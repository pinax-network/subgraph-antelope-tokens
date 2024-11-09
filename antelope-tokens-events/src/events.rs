use crate::{balance::insert_balance, pb::antelope::tokens::v1::Events, supply::insert_supply};
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_antelope::pb::DbOps;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, db_ops: DbOps) -> Result<Events, Error> {
    let mut events = Events::default();

    // Events
    for db_op_ext in db_ops.db_ops.iter() {
        let table_name = db_op_ext.table_name.as_str();
        // Events::Balance
        if table_name == "accounts" {
            insert_balance(&clock, &mut events, db_op_ext);
        }
        // Events::Supply
        if table_name == "stat" {
            insert_supply(&clock, &mut events, db_op_ext);
        }
    }

    Ok(events)
}
