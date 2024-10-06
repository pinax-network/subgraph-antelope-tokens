use antelope::{Asset, ExtendedSymbol, Name};
use substreams::pb::substreams::Clock;
use substreams_antelope::decoder::decode;
use substreams_antelope::pb::db_op::Operation;
use substreams_antelope::pb::DbOp;
use substreams_entity_change::tables::Tables;

use crate::abi;
use crate::keys::balance_key;
use crate::order_by::insert_order_by;
use crate::tokens::insert_token_metadata;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L702
// https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/include/eosio.token/eosio.token.hpp#L156-L160
pub fn insert_balance(tables: &mut Tables, clock: &Clock, db_op: &DbOp) -> Option<ExtendedSymbol> {
    // db_op
    let code = Name::from(db_op.code.as_str());
    let owner = Name::from(db_op.scope.as_str());
    let is_deleted = db_op.operation() == Operation::Remove;

    // decoded
    let old_data = decode::<abi::types::Account>(&db_op.old_data_json).ok();
    let new_data = decode::<abi::types::Account>(&db_op.new_data_json).ok();

    // no valid Accounts
    if old_data.is_none() && new_data.is_none() {
        return None;
    }
    // parse Assets
    let old_balance = old_data.as_ref().and_then(|data| match data.balance.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(_e) => None,
    });
    let new_balance = new_data.as_ref().and_then(|data| match data.balance.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(_e) => None,
    });
    // no valid Assets
    if old_balance.is_none() && new_balance.is_none() {
        return None;
    }

    // fields derived from old_balance or new_balance
    let sym = old_balance.or(new_balance).as_ref().expect("missing old_balance or new_balance").symbol;
    let token = ExtendedSymbol::from_extended(sym, code);
    let zero = Asset::from_amount(0, sym);
    let balance = new_balance.as_ref().unwrap_or(&zero);

    // TABLE::Balance
    let key = balance_key(&token, &owner);
    let row = tables
        .create_row("Balance", key)
        // // pointers
        // .set("block", clock.id.as_str())
        // .set("token", &token.to_string())
        // delete mutations
        .set("is_deleted", is_deleted)
        // balance
        .set("owner", owner.to_string())
        .set_bigdecimal("balance", &balance.value().to_string());

    insert_token_metadata(row, &token);
    insert_order_by(row, clock);

    return Some(token);
}
