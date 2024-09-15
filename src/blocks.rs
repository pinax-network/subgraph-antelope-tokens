use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;

use crate::utils::{block_date_to_month, block_date_to_year, block_time_to_date};

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L21
pub fn insert_blocks(tables: &mut Tables, clock: &Clock) {
    // timestamp
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    let date = block_time_to_date(timestamp.to_string().as_str());
    let month = block_date_to_month(date.as_str());
    let year = block_date_to_year(date.as_str());

    // TABLE::Block
    tables
        .create_row("Block", &clock.id)
        .set("date", date)
        .set("month", month)
        .set("year", year)
        .set("timestamp", timestamp)
        .set_bigint("number", &clock.number.to_string());
}
