use substreams::pb::substreams::Clock;
use substreams_antelope::pb::TransactionTrace;
use substreams_entity_change::tables::Tables;

use crate::db_ops::collapse_db_ops;

use super::db_ops::insert_db_op;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L525
pub fn insert_transaction(params: &str, tables: &mut Tables, clock: &Clock, transaction: &TransactionTrace) -> bool {
    // only include successful transactions
    let header = transaction.receipt.as_ref().expect("receipt missing");
    if header.status != 1 {
        return false;
    }

    // collapse overlapping db_ops per transactions (usually spam related contracts)
    let collapsed_db_ops = collapse_db_ops(transaction);

    // TABLE::DbOps
    let mut is_matched = false;
    for db_op_ext in collapsed_db_ops.iter() {
        if insert_db_op(params, tables, clock, &db_op_ext.db_op, transaction, db_op_ext.index) {
            is_matched = true;
        }
    }
    return is_matched;
}
