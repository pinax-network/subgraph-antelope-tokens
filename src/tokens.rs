use antelope::ExtendedSymbol;
use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;

pub fn insert_token(tables: &mut Tables, clock: &Clock, token: &ExtendedSymbol) {
    // TABLE::Token
    let code = token.get_contract();
    let sym = token.get_symbol();
    let precision = sym.precision();

    // TABLE::Token
    tables
        .create_row("Token", token.to_string())
        // deriveFrom
        .set("block", clock.id.as_str())
        // token
        .set("code", code.to_string())
        .set("symcode", sym.code().to_string())
        .set("sym", sym.to_string())
        .set_bigint_or_zero("precision", &precision.to_string());
}
