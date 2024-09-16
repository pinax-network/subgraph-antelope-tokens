use antelope::SymbolCode;

pub fn balance_key(token: &str, owner: &str) -> String {
    format!("{}:{}", token, owner)
}

pub fn token_key(code: &str, symcode: &SymbolCode) -> String {
    format!("{}:{}", code, symcode.to_string())
}
