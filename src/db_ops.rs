use antelope::ExtendedSymbol;
use substreams::{matches_keys_in_parsed_expr, pb::substreams::Clock};
use substreams_antelope::pb::DbOp;
use substreams_entity_change::tables::Tables;

use crate::{balance::insert_balance, supply::insert_supply};

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
pub fn insert_db_op(params: &str, tables: &mut Tables, clock: &Clock, db_op: &DbOp) -> Option<ExtendedSymbol> {
    let table_name = db_op.table_name.as_str();

    // match custom smart contracts `code`
    let keys = vec![format! {"code:{}", db_op.code.to_string()}];
    if !params.is_empty() && matches_keys_in_parsed_expr(&keys, params).unwrap_or(false) {
        return None;
    }

    // TABLE::Balance
    if table_name == "accounts" {
        return insert_balance(tables, clock, db_op);
    }
    // TABLE::Supply
    if table_name == "stat" {
        return insert_supply(tables, clock, db_op);
    }
    return None;
}
