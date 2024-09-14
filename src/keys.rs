use antelope::SymbolCode;

pub fn db_ops_table_key(code: &str, scope: &str, table_name: &str, primary_key: &str) -> String {
    format!("{}:{}:{}:{}", code, scope, table_name, primary_key)
}

pub fn balance_key(owner: &str, token: &str) -> String {
    format!("{}:{}", owner, token)
}

pub fn token_key(symcode: &SymbolCode, code: &str) -> String {
    format!("{}:{}", symcode.to_string(), code)
}
