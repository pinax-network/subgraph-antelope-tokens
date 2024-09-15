use std::collections::HashMap;

use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;
use substreams_entity_change::pb::entity::EntityChanges;

use crate::blocks::insert_blocks;
use crate::db_ops::{collapse_db_ops_by_block, insert_db_op};
use crate::tokens::insert_token;

#[substreams::handlers::map]
pub fn graph_out(params: String, clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    // collapse overlapping db_ops per transactions
    // only stores "last token update" per block
    // if multiple db_ops of same token, usually spam related contracts
    let collapsed_db_ops = collapse_db_ops_by_block(&block);

    // TABLE::Balance,Supply
    let mut tokens = HashMap::new();
    for db_op_ext in collapsed_db_ops.iter() {
        match insert_db_op(&params, &mut tables, &clock, &db_op_ext) {
            Some(token) => tokens.insert(token.key.clone(), token),
            None => None,
        };
    }

    // TABLE::Token
    for token in tokens.values() {
        insert_token(&mut tables, token);
    }

    // TABLE::blocks
    if !tokens.is_empty() {
        insert_blocks(&mut tables, &clock);
    }

    Ok(tables.to_entity_changes())
}
