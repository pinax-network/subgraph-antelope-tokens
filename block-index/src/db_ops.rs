use std::collections::HashMap;

use substreams::errors::Error;
use substreams_antelope::{
    pb::{DbOp, DbOps},
    Block,
};

#[derive(Debug, Clone)]
pub struct DbOpExt {
    pub index: u32,
    pub transaction_id: String,
    pub db_op: DbOp,
}

pub fn db_ops_table_key(code: &str, scope: &str, table_name: &str, primary_key: &str) -> String {
    format!("{}:{}:{}:{}", code, scope, table_name, primary_key)
}

pub fn collapse_db_ops(db_ops: &Vec<DbOp>) -> Vec<DbOp> {
    let mut collapsed_db_ops: HashMap<String, DbOp> = HashMap::new();
    for db_op in db_ops.iter() {
        let code = db_op.code.as_str();
        let scope = db_op.scope.as_str();
        let table_name = db_op.table_name.as_str();
        let primary_key = db_op.primary_key.as_str();
        let table_key = db_ops_table_key(code, scope, table_name, primary_key);

        // first db ops, no need to inherit from previous db ops
        if !collapsed_db_ops.contains_key(&table_key) {
            collapsed_db_ops.insert(table_key, db_op.clone());
        // inherit from previous db ops
        // new_data and new_data_json are updated
        } else {
            let collapsed_db_op = collapsed_db_ops.get_mut(&table_key).unwrap();
            collapsed_db_op.new_data = db_op.new_data.clone();
            collapsed_db_op.new_data_json = db_op.new_data_json.clone();
            collapsed_db_op.operation = db_op.operation;
        }
    }
    collapsed_db_ops.into_values().collect()
}

// https://github.com/pinax-network/firehose-antelope/blob/b6d200b0330671f8d65df4cd3b29b8c33ca2b365/proto/sf/antelope/type/v1/type.proto#L702
#[substreams::handlers::map]
pub fn map_db_ops(block: Block) -> Result<DbOps, Error> {
    let mut db_ops: Vec<DbOp> = vec![];

    // filter by table_name
    for transaction in block.transaction_traces() {
        for db_op in transaction.db_ops.iter() {
            if db_op.table_name == "accounts" || db_op.table_name == "stat" {
                db_ops.push(db_op.clone());
            }
        }
    }

    Ok(DbOps { db_ops: collapse_db_ops(&db_ops) })
}
