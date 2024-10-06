use antelope::ExtendedSymbol;
use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::{Row, Tables};

use crate::order_by::insert_order_by;

pub fn insert_token(tables: &mut Tables, clock: &Clock, token: &ExtendedSymbol) {
    // TABLE::Token
    let row = tables.create_row("Token", token.to_string());

    insert_token_metadata(row, token);
    insert_order_by(row, clock);
}

pub fn insert_token_metadata(row: &mut Row, token: &ExtendedSymbol) {
    let code = token.get_contract();
    let sym = token.get_symbol();
    let precision = sym.precision();

    // TABLE::Token
    row.set("code", code.to_string())
        .set("symcode", sym.code().to_string())
        .set("sym", sym.to_string())
        .set_bigint_or_zero("precision", &precision.to_string());
}
