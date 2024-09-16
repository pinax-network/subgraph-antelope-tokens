use substreams::pb::substreams::Clock;
use substreams_antelope::pb::DbOp;
use substreams_entity_change::tables::Tables;

use crate::{balance::insert_balance, index::collect_db_op_keys, supply::insert_supply, tokens::Token, utils::is_match};

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
pub fn insert_db_op(params: &str, tables: &mut Tables, clock: &Clock, db_op: &DbOp) -> Option<Token> {
    let table_name = db_op.table_name.as_str();

    if is_match(collect_db_op_keys(db_op), params) {
        // TABLE::Balance
        if table_name == "accounts" {
            return insert_balance(tables, clock, db_op);
        }
        // TABLE::Supply
        if table_name == "stat" {
            return insert_supply(tables, clock, db_op);
        }
    }
    return None;
}
