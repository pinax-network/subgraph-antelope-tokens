use antelope::{Asset, ExtendedSymbol, Name};
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::DbOp;

use crate::{
    pb::antelope::tokens::v1::{Events, Supply},
    utils::{parse_json_asset, parse_json_name},
};

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L702
// https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/include/eosio.token/eosio.token.hpp#L162-L168
pub fn insert_supply(clock: &Clock, events: &mut Events, db_op: &DbOp) {
    // db_op
    let code = Name::from(db_op.code.as_str());

    // parse Assets
    let old_supply = parse_json_asset(&db_op.old_data_json, "supply");
    let new_supply = parse_json_asset(&db_op.new_data_json, "supply");
    let new_max_supply = parse_json_asset(&db_op.new_data_json, "max_supply");
    let new_issuer = parse_json_name(&db_op.new_data_json, "issuer");

    // no valid Assets
    if old_supply.is_none() && new_supply.is_none() {
        return;
    }

    // fields derived from old_balance or new_balance
    let sym = old_supply.or(new_supply).as_ref().expect("missing old_supply or new_supply").symbol;
    let token = ExtendedSymbol::from_extended(sym, code);
    let zero = Asset::from_amount(0, sym);
    let supply = new_supply.as_ref().unwrap_or(&zero);
    let max_supply = new_max_supply.as_ref().unwrap_or(&zero);
    let issuer = new_issuer.unwrap_or(Name::new());

    // Supply
    events.supply_events.push(Supply {
        block_time: clock.timestamp,
        block_number: clock.number,
        block_hash: clock.id.to_string(),
        token: token.to_string(),
        supply: supply.amount,
        max_supply: max_supply.amount,
        issuer: issuer.to_string(),
        operation: db_op.operation().as_str_name().to_string(),
    });
}
