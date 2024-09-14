use antelope::{Symbol, SymbolCode};

pub fn action_key(tx_hash: &str, index: u32) -> String {
    format!("{}:{}", tx_hash, index)
}

pub fn db_ops_key(tx_hash: &str, execution_index: u32, db_op_index: u32) -> String {
    format!("{}:{}:{}", tx_hash, execution_index, db_op_index)
}

pub fn db_ops_table_key(code: &str, scope: &str, table_name: &str, primary_key: &str) -> String {
    format!("{}:{}:{}:{}", code, scope, table_name, primary_key)
}

pub fn balance_key(owner: &str, token: &str) -> String {
    format!("{}:{}", owner, token)
}

pub fn token_key(symcode: &SymbolCode, code: &str) -> String {
    format!("{}@{}", symcode.to_string(), code)
}
