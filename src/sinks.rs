use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;
use substreams_entity_change::pb::entity::EntityChanges;

use crate::blocks::insert_blocks;
use crate::db_ops::{collapse_db_ops_by_block, insert_db_op};

#[substreams::handlers::map]
pub fn graph_out(params: String, clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    // collapse overlapping db_ops per transactions
    // only stores "last token update" per block
    // if multiple db_ops of same token, usually spam related contracts
    let table_names = vec!["accounts", "stats"];
    let collapsed_db_ops = collapse_db_ops_by_block(&block, table_names);

    // TABLE::Balance,Token,Supply
    let mut matched = false;
    for db_op_ext in collapsed_db_ops.iter() {
        if insert_db_op(&params, &mut tables, &clock, &db_op_ext.db_op) {
            matched = true;
        }
    }

    // TABLE::blocks
    if matched {
        insert_blocks(&mut tables, &clock);
    }

    Ok(tables.to_entity_changes())
}
