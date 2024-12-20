use std::collections::HashSet;
use std::str::FromStr;

use antelope::{ExtendedSymbol, Symbol};
use antelope_tokens_events::pb::antelope::tokens::v1::Events;
use substreams::errors::Error;
use substreams::matches_keys_in_parsed_expr;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::db_op::Operation;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

fn to_float64(amount: i64, sym: Symbol) -> f64 {
    amount as f64 / 10_f64.powi(sym.precision() as i32)
}

#[substreams::handlers::map]
pub fn graph_out(params: String, clock: Clock, events: Events) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let mut tokens = HashSet::new();

    // TABLE::Balance
    for event in events.balance_events {
        if !match_token(&params, &event.token) {
            continue;
        }
        let key = format!("{}:{}", event.token, event.owner);
        let ext_sym = ExtendedSymbol::from_str(&event.token).expect("invalid token ExtendedSymbol");
        let sym = ext_sym.get_symbol();
        let balance = to_float64(event.balance, sym);
        // INSERT
        if event.operation == Operation::Insert.as_str_name() {
            tables
                .create_row("Balance", &key)
                // deriveFrom
                .set("block", clock.id.as_str())
                .set("token", event.token.as_str())
                // balance
                .set("owner", event.owner.to_string())
                .set_bigdecimal("balance", &balance.to_string());
        }
        // UPDATE
        else if event.operation == Operation::Update.as_str_name() {
            tables.update_row("Balance", &key).set_bigdecimal("balance", &balance.to_string());
        }
        // DELETE
        else if event.operation == Operation::Remove.as_str_name() {
            tables.delete_row("Balance", &key);
        }
        tokens.insert(event.token);
    }

    // TABLE::Supply
    for event in events.supply_events {
        if !match_token(&params, &event.token) {
            continue;
        }
        let ext_sym = ExtendedSymbol::from_str(&event.token).expect("invalid token ExtendedSymbol");
        let sym = ext_sym.get_symbol();
        let supply = to_float64(event.supply, sym);
        let max_supply = to_float64(event.max_supply, sym);
        // INSERT
        if event.operation == Operation::Insert.as_str_name() {
            tables
                .create_row("Supply", event.token.as_str())
                // deriveFrom
                .set("block", clock.id.as_str())
                .set("token", event.token.as_str())
                // supply
                .set_bigdecimal("supply", &supply.to_string())
                .set_bigdecimal("max_supply", &max_supply.to_string())
                .set("issuer", &event.issuer.to_string());

            // TABLE::Token
            tables
                .create_row("Token", event.token)
                // deriveFrom
                .set("block", clock.id.as_str())
                // token
                .set("contract", ext_sym.get_contract().to_string())
                .set("symcode", sym.code().to_string())
                .set("sym", sym.to_string())
                .set_bigint_or_zero("precision", &sym.precision().to_string());
        // UPDATE
        } else if event.operation == Operation::Update.as_str_name() {
            tables
                .update_row("Supply", event.token.as_str())
                .set_bigdecimal("supply", &supply.to_string())
                .set_bigdecimal("max_supply", &max_supply.to_string())
                .set("issuer", &event.issuer.to_string());
        // DELETE
        } else if event.operation == Operation::Remove.as_str_name() {
            tables.delete_row("Supply", event.token.as_str());
        }
    }

    // TABLE::Token
    for token in tokens.iter() {
        let ext_sym = ExtendedSymbol::from_str(token).expect("invalid token ExtendedSymbol");
        let sym = ext_sym.get_symbol();
        tables
            .create_row("Token", token)
            // deriveFrom
            .set("block", clock.id.as_str())
            // token
            .set("contract", ext_sym.get_contract().to_string())
            .set("symcode", sym.code().to_string())
            .set("sym", sym.to_string())
            .set_bigint_or_zero("precision", &sym.precision().to_string());
    }

    // if no rows, skip Block by return empty EntityChanges
    if tables.tables.is_empty() {
        return Ok(EntityChanges::default());
    };

    // TABLE::Block
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    let date = to_date(&clock);

    // TABLE::Block
    tables
        .create_row("Block", &clock.id)
        .set_bigint("number", &clock.number.to_string())
        .set("date", date)
        // .set("timestamp", timestamp) // Not yet supported by latest Graph Node
        .set_bigint("seconds", &timestamp.seconds.to_string());

    Ok(tables.to_entity_changes())
}

// Clock to date string
// ex: Clock => 2015-07-30
pub fn to_date(clock: &Clock) -> String {
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    timestamp.to_string().split('T').next().expect("missing date").to_string()
}

pub fn match_token(params: &str, token: &str) -> bool {
    if params.is_empty() {
        return true;
    }
    let ext_sym = ExtendedSymbol::from_str(token).expect("invalid ExtendedSymbol");
    let keys = vec![format!("contract:{}", ext_sym.get_contract()), format!("token:{}", ext_sym)];
    match matches_keys_in_parsed_expr(&keys, params) {
        Ok(true) => return true,
        Ok(false) => return false,
        Err(e) => panic!("{}", e),
    }
}
