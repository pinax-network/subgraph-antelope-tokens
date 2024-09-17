use std::collections::HashMap;

use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::DbOps;
use substreams_entity_change::pb::entity::EntityChanges;

use crate::blocks::insert_blocks;
use crate::db_ops::insert_db_op;
use crate::tokens::insert_token;

#[substreams::handlers::map]
pub fn graph_out(params: String, clock: Clock, db_ops: DbOps) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    // TABLE::Balance,Supply
    let mut tokens = HashMap::new();
    for db_op_ext in db_ops.db_ops.iter() {
        match insert_db_op(&params, &mut tables, &clock, &db_op_ext) {
            Some(token) => tokens.insert(token.to_string(), token),
            None => None,
        };
    }

    // TABLE::Token
    for token in tokens.values() {
        insert_token(&mut tables, &clock, token);
    }

    // TABLE::Block
    if !tokens.is_empty() {
        insert_blocks(&mut tables, &clock);
    }

    Ok(tables.to_entity_changes())
}
