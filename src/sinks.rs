use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;
use substreams_entity_change::pb::entity::EntityChanges;

use crate::blocks::insert_blocks;
use crate::transactions::insert_transaction;

#[substreams::handlers::map]
pub fn graph_out(params: String, clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    // TABLE::blocks
    insert_blocks(&mut tables, &clock);

    // TABLE::Balance,Token,Supply
    for transaction in block.transaction_traces() {
        insert_transaction(&params, &mut tables, &clock, &transaction);
    }

    Ok(tables.to_entity_changes())
}
