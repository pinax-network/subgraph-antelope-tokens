use antelope::{Asset, Name, Symbol, SymbolCode};
use substreams::log;
use substreams::pb::substreams::Clock;
use substreams_antelope::decoder::decode;
use substreams_antelope::pb::db_op::Operation;
use substreams_antelope::pb::{DbOp, TransactionTrace};
use substreams_entity_change::tables::Tables;

use crate::abi;
use crate::keys::{action_key, db_ops_key, supply_key, token_key};

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L702
// https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/include/eosio.token/eosio.token.hpp#L162-L168
pub fn insert_supply(tables: &mut Tables, clock: &Clock, db_op: &DbOp, transaction: &TransactionTrace, index: u32) -> bool {
    // db_op
    let code = db_op.code.as_str();
    // let scope = db_op.scope.as_str();
    let table_name = db_op.table_name.as_str();
    let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
    let symcode = SymbolCode::from(raw_primary_key);
    let key = supply_key(&code, &symcode);

    // supply rows are typically stored in the "stat" table
    if table_name != "stat" {
        return false;
    }

    // removal of balance typically handled by `close` action
    // https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/src/eosio.token.cpp#L182
    // if db_op.operation() == Operation::Remove {
    //     tables.delete_row("Supply", &key);
    // }

    // decoded
    let old_data = decode::<abi::types::Account>(&db_op.old_data_json).ok();
    let new_data = decode::<abi::types::Account>(&db_op.new_data_json).ok();

    let old_supply = old_data.as_ref().and_then(|data| match data.balance.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(e) => {
            log::info!("Error parsing old balance asset in transaction {}: {:?}", transaction.id, e);
            None
        }
    });
    let new_supply = new_data.as_ref().and_then(|data| match data.balance.parse::<Asset>() {
        Ok(asset) => Some(asset),
        Err(e) => {
            log::info!("Error parsing new balance asset in transaction {}: {:?}", transaction.id, e);
            None
        }
    });

    // no balance changes
    if old_supply.is_none() && new_supply.is_none() {
        return false;
    }

    let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
    let symcode = SymbolCode::from(raw_primary_key);
    let precision = new_supply.unwrap_or_else(|| old_supply.unwrap()).symbol.precision();
    let sym = Symbol::from_precision(symcode, precision);
    let supply = new_supply.unwrap_or_else(|| Asset::from_amount(0, sym));
    // let supply_delta = supply.amount - old_supply.unwrap_or_else(|| Asset::from_amount(0, sym)).amount;

    // pointers
    let tx_hash = transaction.id.as_str();
    let action_index = db_op.action_index;
    let token = token_key(&sym, code);
    let db_op = db_ops_key(tx_hash, action_index, index);

    // TABLE::Balance
    let action_key = action_key(tx_hash, action_index);
    tables
        .create_row("Balance", key)
        // pointers
        .set("block", clock.id.as_str())
        // pointers for Antelope Transactions
        // .set("transaction", tx_hash)
        // .set("action", action_key)
        // .set("token", token)
        // .set("dbOp", db_op)
        // balance
        .set("supply", supply.to_string())
        .set("code", code)
        .set("symcode", sym.code().to_string())
        .set_bigint("precision", &precision.to_string())
        .set_bigdecimal("value", &supply.value().to_string())
        .set_bigint_or_zero("amount", &supply.amount.to_string());
    return true;
}
