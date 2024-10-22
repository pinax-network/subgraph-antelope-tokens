use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;

use crate::utils::to_date;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L21
pub fn insert_blocks(tables: &mut Tables, clock: &Clock) {
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    let date = to_date(clock);

    // TABLE::Block
    tables
        .create_row("Block", &clock.id)
        .set_bigint("number", &clock.number.to_string())
        .set("date", date)
        // .set("timestamp", timestamp) // Not yet supported by latest Graph Node
        .set_bigint("seconds", &timestamp.seconds.to_string());
}
