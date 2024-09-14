use antelope::{Asset, Name, Symbol, SymbolCode};
use substreams::log;
use substreams::pb::substreams::Clock;
use substreams_antelope::decoder::decode;
use substreams_antelope::pb::db_op::Operation;
use substreams_antelope::pb::DbOp;
use substreams_entity_change::tables::Tables;

use crate::abi;
use crate::keys::{balance_key, token_key};

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L702
// https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/include/eosio.token/eosio.token.hpp#L156-L160
pub fn insert_balance(tables: &mut Tables, clock: &Clock, db_op: &DbOp) -> bool {
    // db_op
    let code = db_op.code.as_str();
    let owner = db_op.scope.as_str();
    let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
    let symcode = SymbolCode::from(raw_primary_key);
    let token = token_key(&symcode, code);
    let key = balance_key(&owner, &token);

    // removal of balance typically handled by `close` action
    // https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/src/eosio.token.cpp#L182
    if db_op.operation() == Operation::Remove {
        // TABLE::Balance
        tables
            .create_row("Balance", token.as_str())
            // pointers
            .set("block", clock.id.as_str())
            .set("token", token.as_str())
            .set("owner", owner)
            // balance
            .set_bigdecimal("value", &0.to_string())
            .set_bigint_or_zero("amount", &0.to_string());
    }

    // decoded
    let old_data = decode::<abi::types::Account>(&db_op.old_data_json).ok();
    let new_data = decode::<abi::types::Account>(&db_op.new_data_json).ok();

    let old_balance = old_data.as_ref().and_then(|data| match data.balance.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(e) => {
            log::info!("Error parsing old balance asset in block {}: {:?}", clock.number, e);
            None
        }
    });
    let new_balance = new_data.as_ref().and_then(|data| match data.balance.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(e) => {
            log::info!("Error parsing new balance asset in block {}: {:?}", clock.number, e);
            None
        }
    });

    // balance has been removed
    if new_balance.is_none() {
        return false;
    }

    let precision = new_balance.unwrap_or_else(|| old_balance.unwrap()).symbol.precision();
    let sym = Symbol::from_precision(symcode, precision);
    let balance = new_balance.unwrap_or_else(|| Asset::from_amount(0, sym));

    // TABLE::Balance
    tables
        .create_row("Balance", key)
        // pointers
        .set("block", clock.id.as_str())
        .set("token", token)
        // balance
        .set("owner", owner)
        .set_bigdecimal("value", &balance.value().to_string())
        .set_bigint_or_zero("amount", &balance.amount.to_string());

    return true;
}
