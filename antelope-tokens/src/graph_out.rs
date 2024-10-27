use std::collections::HashSet;
use std::str::FromStr;

use antelope::ExtendedSymbol;
use substreams::errors::Error;
use substreams::matches_keys_in_parsed_expr;
use substreams::pb::substreams::Clock;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

use crate::pb::antelope::tokens::v1::Events;

#[substreams::handlers::map]
pub fn graph_out(params: String, clock: Clock, events: Events) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let mut tokens = HashSet::new();

    // TABLE::Balance
    for balance in events.balance_events {
        let ext_sym = ExtendedSymbol::from_str(&balance.token).expect("invalid ExtendedSymbol");
        let sym = ext_sym.get_symbol();
        let keys = vec![
            format!("code:{}", ext_sym.get_contract()),
            format!("owner:{}", balance.owner),
            format!("sym:{}", sym),
            format!("symcode:{}", sym.code()),
            format!("token:{}", ext_sym),
        ];
        match matches_keys_in_parsed_expr(&keys, &params) {
            Ok(true) => {}
            Ok(false) => continue,
            Err(e) => return Err(Error::from(e)),
        }
        let key = format!("{}:{}", balance.token, balance.owner);
        tables
            .create_row("Balance", key)
            // deriveFrom
            .set("block", clock.id.as_str())
            .set("token", &balance.token.to_string())
            // delete mutations
            .set("is_deleted", balance.is_deleted)
            // balance
            .set("owner", balance.owner.to_string())
            .set_bigdecimal("balance", &balance.balance.to_string());
        tokens.insert(balance.token);
    }

    // TABLE::Supply
    for supply in events.supply_events {
        let ext_sym = ExtendedSymbol::from_str(&supply.token).expect("invalid ExtendedSymbol");
        let sym = ext_sym.get_symbol();
        let keys = vec![
            format!("code:{}", ext_sym.get_contract()),
            format!("sym:{}", sym),
            format!("symcode:{}", sym.code()),
            format!("token:{}", ext_sym),
        ];
        match matches_keys_in_parsed_expr(&keys, &params) {
            Ok(true) => {}
            Ok(false) => continue,
            Err(e) => return Err(Error::from(e)),
        }
        tables
            .create_row("Supply", supply.token.to_string().as_str())
            // deriveFrom
            .set("block", clock.id.as_str())
            .set("token", supply.token.to_string().as_str())
            // delete mutations
            .set("is_deleted", supply.is_deleted)
            // supply
            .set_bigdecimal("supply", &supply.supply.to_string())
            .set_bigdecimal("max_supply", &supply.max_supply.to_string())
            .set("issuer", &supply.issuer.to_string());
        tokens.insert(supply.token);
    }

    // TABLE::Token
    for token in tokens.iter() {
        let ext_sym = ExtendedSymbol::from_str(token).expect("invalid ExtendedSymbol");
        let sym = ext_sym.get_symbol();
        tables
            .create_row("Token", token)
            // deriveFrom
            .set("block", clock.id.as_str())
            // token
            .set("code", ext_sym.get_contract().to_string())
            .set("symcode", sym.code().to_string())
            .set("sym", sym.to_string())
            .set_bigint_or_zero("precision", &sym.precision().to_string());
    }

    // if no tokens, skip block by return empty EntityChanges
    if tokens.is_empty() {
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
