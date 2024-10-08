use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Row;

use crate::utils::to_date;

pub fn insert_order_by(row: &mut Row, clock: &Clock) {
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    let block_number = clock.number.to_string();
    let block_date = to_date(clock);

    row.set("timestamp", timestamp).set_bigint("block_number", &block_number.to_string()).set("block_date", &block_date);
}
