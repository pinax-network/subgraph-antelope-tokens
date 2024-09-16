use antelope::{Asset, Name, Symbol, SymbolCode};
use substreams::log;
use substreams::pb::substreams::Clock;
use substreams_antelope::decoder::decode;
use substreams_antelope::pb::db_op::Operation;
use substreams_antelope::pb::DbOp;
use substreams_entity_change::tables::Tables;

use crate::abi;
use crate::keys::token_key;
use crate::tokens::Token;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L702
// https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/include/eosio.token/eosio.token.hpp#L162-L168
pub fn insert_supply(tables: &mut Tables, clock: &Clock, db_op: &DbOp) -> Option<Token> {
    // db_op
    let code = db_op.code.as_str();
    let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
    let symcode = SymbolCode::from(raw_primary_key);
    let token = token_key(code, &symcode);
    let block = clock.id.as_str();

    // removal of balance typically handled by `close` action
    // https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/src/eosio.token.cpp#L182
    if db_op.operation() == Operation::Remove {
        // TABLE::Supply
        tables
            .create_row("Supply", token.as_str())
            // pointers
            .set("block", block)
            .set("token", token.as_str())
            // supply
            .set_bigdecimal("supply", &0.to_string())
            .set_bigdecimal("maxSupply", &0.to_string());
        return None;
    }

    // decoded
    // let old_data = decode::<abi::types::CurrencyStats>(&db_op.old_data_json).ok();
    let new_data = decode::<abi::types::CurrencyStats>(&db_op.new_data_json).ok();

    let new_supply = new_data.as_ref().and_then(|data| match data.supply.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(e) => {
            log::info!("Error parsing new supply asset in block {}: {:?}", clock.number, e);
            None
        }
    });

    let new_max_supply = new_data.as_ref().and_then(|data| match data.max_supply.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(e) => {
            log::info!("Error parsing new supply asset in block {}: {:?}", clock.number, e);
            None
        }
    });

    // supply has been removed
    if new_supply.is_none() || new_max_supply.is_none() {
        return None;
    }

    let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
    let symcode = SymbolCode::from(raw_primary_key);
    let supply = new_supply.as_ref().expect("missing new_supply");
    let max_supply = new_max_supply.as_ref().expect("missing new_supply");
    let precision = supply.symbol.precision();
    let sym = Symbol::from_precision(symcode, precision);

    // TABLE::Supply
    tables
        .create_row("Supply", token.as_str())
        // pointers
        .set("block", block)
        .set("token", token.as_str())
        // supply
        .set_bigdecimal("supply", &supply.value().to_string())
        .set_bigdecimal("maxSupply", &max_supply.value().to_string());

    return Some(Token {
        key: token.to_string(),
        clock: clock.clone(),
        code: code.to_string(),
        sym: sym,
    });
}
