use crate::pb::antelope::tokens::v1::{Balance, Events};
use crate::utils::parse_json_asset;
use antelope::{Asset, ExtendedSymbol, Name};
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::db_op::Operation;
use substreams_antelope::pb::DbOp;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L702
// https://github.com/eosnetworkfoundation/eos-system-contracts/blob/8ecd1ac6d312085279cafc9c1a5ade6affc886da/contracts/eosio.token/include/eosio.token/eosio.token.hpp#L156-L160
pub fn insert_balance(clock: &Clock, events: &mut Events, db_op: &DbOp) {
    // db_op
    let code = Name::from(db_op.code.as_str());
    let owner = Name::from(db_op.scope.as_str());

    // decoded
    let old_balance = parse_json_asset(&db_op.old_data_json, "balance");
    let new_balance = parse_json_asset(&db_op.new_data_json, "balance");

    // no valid Accounts
    if old_balance.is_none() && new_balance.is_none() {
        return;
    }

    // if modified & balance has not changed, ignore
    if db_op.operation() == Operation::Update && old_balance == new_balance {
        return;
    }

    // fields derived from old_balance or new_balance
    let sym = old_balance.or(new_balance).as_ref().expect("missing old_balance or new_balance").symbol;
    let token = ExtendedSymbol::from_extended(sym, code);
    let zero = Asset::from_amount(0, sym);
    let balance = new_balance.as_ref().unwrap_or(&zero);

    // Balance
    events.balance_events.push(Balance {
        block_time: clock.timestamp,
        block_number: clock.number,
        block_hash: clock.id.to_string(),
        token: token.to_string(),
        owner: owner.to_string(),
        balance: balance.amount,
        operation: db_op.operation().as_str_name().to_string(),
    });
}
