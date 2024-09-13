use std::collections::HashMap;

use itertools::Itertools as _;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::{DbOp, TransactionTrace};
use substreams_entity_change::tables::Tables;

pub struct DbOpExt {
    pub index: u32,
    pub db_op: DbOp,
}

use crate::{
    balance::insert_balance,
    index::{collect_db_op_keys, is_match},
    keys::db_ops_table_key,
    supply::insert_supply,
};

pub fn collapse_db_ops(transaction: &TransactionTrace) -> Vec<DbOpExt> {
    let mut collapsed_db_ops: HashMap<String, DbOpExt> = HashMap::new();
    let mut index = 0;
    for db_op in transaction.db_ops.iter() {
        let code = db_op.code.as_str();
        let scope = db_op.scope.as_str();
        let table_name = db_op.table_name.as_str();
        let primary_key = db_op.primary_key.as_str();
        let table_key = db_ops_table_key(code, scope, table_name, primary_key);

        // first db ops, no need to inherit from previous db ops
        if !collapsed_db_ops.contains_key(&table_key) {
            collapsed_db_ops.insert(table_key, DbOpExt { db_op: db_op.clone(), index });
        // inherit from previous db ops
        // new_data and new_data_json are updated
        } else {
            let collapsed_db_op = collapsed_db_ops.get_mut(&table_key).unwrap();
            collapsed_db_op.db_op.new_data = db_op.new_data.clone();
            collapsed_db_op.db_op.new_data_json = db_op.new_data_json.clone();
            collapsed_db_op.db_op.operation = db_op.operation;
        }
        index += 1;
    }
    collapsed_db_ops
        .into_values()
        .sorted_by_key(|db_op_ext| db_op_ext.index) // Sorts by action_index
        .collect()
}

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
pub fn insert_db_op(params: &str, tables: &mut Tables, clock: &Clock, db_op: &DbOp, transaction: &TransactionTrace, index: u32) -> bool {
    let table_name = db_op.table_name.as_str();

    if is_match(collect_db_op_keys(db_op), params) {
        // TABLE::Balance
        if table_name == "accounts" {
            return insert_balance(tables, clock, db_op, transaction, index);
        }
        // TABLE::Supply
        if table_name == "stat" {
            return insert_supply(tables, clock, db_op, transaction, index);
        }
    }
    return false;
}
