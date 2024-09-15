use antelope::Symbol;
use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;

use crate::keys::token_key;

#[derive(Debug, Clone)]
pub struct Token {
    pub key: String,
    pub clock: Clock,
    pub code: String,
    pub sym: Symbol,
}

pub fn insert_token(tables: &mut Tables, token: &Token) {
    let key = token_key(&token.code, &token.sym.code());

    // TABLE::Token
    tables
        .create_row("Token", key)
        // pointers
        .set("block", token.clock.id.as_str())
        // Token
        .set("code", token.code.as_str())
        .set("symcode", token.sym.code().to_string())
        .set("sym", token.sym.to_string())
        .set_bigint_or_zero("precision", &token.sym.precision().to_string());
}
