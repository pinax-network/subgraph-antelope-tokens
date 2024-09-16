use antelope::{Asset, Name, Symbol, SymbolCode};
use substreams::log;
use substreams::pb::substreams::Clock;
use substreams_antelope::decoder::decode;
use substreams_antelope::pb::db_op::Operation;
use substreams_antelope::pb::DbOp;
use substreams_entity_change::tables::Tables;

use crate::abi;
use crate::keys::{balance_key, token_key};
use crate::tokens::Token;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L702
// https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/include/eosio.token/eosio.token.hpp#L156-L160
pub fn insert_balance(tables: &mut Tables, clock: &Clock, db_op: &DbOp) -> Option<Token> {
    // db_op
    let code = db_op.code.as_str();
    let owner = db_op.scope.as_str();
    let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
    let symcode = SymbolCode::from(raw_primary_key);
    let token = token_key(code, &symcode);
    let key = balance_key(&token, &owner);
    let block = clock.id.as_str();

    // // removal of balance typically handled by `close` action
    // https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/src/eosio.token.cpp#L182
    if db_op.operation() == Operation::Remove {
        log::debug!("REMOVE {}:{}", token, owner);
        // TABLE::Balance
        tables
            .create_row("Balance", &key)
            // pointers
            .set("block", block)
            .set("token", &token)
            // balance
            .set("owner", owner)
            .set_bigdecimal("balance", &0.to_string());
        return None;
    }

    // decoded
    // let old_data = decode::<abi::types::Account>(&db_op.old_data_json).ok();
    let new_data = decode::<abi::types::Account>(&db_op.new_data_json).ok();

    // let old_balance = old_data.as_ref().and_then(|data| match data.balance.parse::<Asset>() {
    //     Ok(asset) => Some(asset),
    //     Err(e) => {
    //         log::info!("Error parsing old balance asset in block {}: {:?}", clock.number, e);
    //         None
    //     }
    // });
    let new_balance = new_data.as_ref().and_then(|data| match data.balance.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(e) => {
            log::info!("Error parsing new balance asset in block {}: {:?}", clock.number, e);
            None
        }
    });

    // balance has been removed
    if new_balance.is_none() {
        return None;
    }

    let balance = new_balance.as_ref().expect("missing new_balance");
    let precision = balance.symbol.precision();
    let sym = Symbol::from_precision(symcode, precision);

    // TABLE::Balance
    tables
        .create_row("Balance", key)
        // pointers
        .set("block", block)
        .set("token", &token)
        // balance
        .set("owner", owner)
        .set_bigdecimal("balance", &balance.value().to_string());

    log::debug!("INSERT: {:?}", balance);
    return Some(Token {
        key: token.to_string(),
        clock: clock.clone(),
        code: code.to_string(),
        sym,
    });
}
