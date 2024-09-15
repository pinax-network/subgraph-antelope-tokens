use antelope::SymbolCode;

pub fn db_ops_table_key(code: &str, scope: &str, table_name: &str, primary_key: &str) -> String {
    format!("{}:{}:{}:{}", code, scope, table_name, primary_key)
}

pub fn balance_key(token: &str, owner: &str) -> String {
    format!("{}:{}", token, owner)
}

pub fn token_key(code: &str, symcode: &SymbolCode) -> String {
    format!("{}:{}", code, symcode.to_string())
}
