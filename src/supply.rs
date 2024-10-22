use antelope::{Asset, ExtendedSymbol, Name};
use substreams::pb::substreams::Clock;
use substreams_antelope::decoder::decode;
use substreams_antelope::pb::db_op::Operation;
use substreams_antelope::pb::DbOp;
use substreams_entity_change::tables::Tables;

use crate::abi;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L702
// https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/include/eosio.token/eosio.token.hpp#L162-L168
pub fn insert_supply(tables: &mut Tables, clock: &Clock, db_op: &DbOp) -> Option<ExtendedSymbol> {
    // db_op
    let code = Name::from(db_op.code.as_str());
    let is_deleted = db_op.operation() == Operation::Remove;

    // decoded
    let old_data = decode::<abi::types::CurrencyStats>(&db_op.old_data_json).ok();
    let new_data = decode::<abi::types::CurrencyStats>(&db_op.new_data_json).ok();

    // no valid Accounts
    if old_data.is_none() && new_data.is_none() {
        return None;
    }
    // parse Assets
    let old_supply = old_data.as_ref().and_then(|data| match data.supply.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(_e) => None,
    });
    let new_supply = new_data.as_ref().and_then(|data| match data.supply.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(_e) => None,
    });
    let new_max_supply = new_data.as_ref().and_then(|data| match data.max_supply.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(_e) => None,
    });
    // no valid Assets
    if old_supply.is_none() && new_supply.is_none() {
        return None;
    }

    // fields derived from old_balance or new_balance
    let sym = old_supply.or(new_supply).as_ref().expect("missing old_supply or new_supply").symbol;
    let token = ExtendedSymbol::from_extended(sym, code);
    let zero = Asset::from_amount(0, sym);
    let supply = new_supply.as_ref().unwrap_or(&zero);
    let max_supply = new_max_supply.as_ref().unwrap_or(&zero);

    // TABLE::Supply
    tables
        .create_row("Supply", token.to_string().as_str())
        // deriveFrom
        .set("block", clock.id.as_str())
        .set("token", token.to_string().as_str())
        // delete mutations
        .set("is_deleted", is_deleted)
        // supply
        .set_bigdecimal("supply", &supply.value().to_string())
        .set_bigdecimal("max_supply", &max_supply.value().to_string());

    return Some(token);
}
