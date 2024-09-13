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

pub fn balance_key(code: &str, scope: &str, symcode: &SymbolCode) -> String {
    format!("{}:{}:{}", code, scope, symcode.to_string())
}

pub fn supply_key(code: &str, symcode: &SymbolCode) -> String {
    format!("{}:{}", code, symcode.to_string())
}

pub fn token_key(sym: &Symbol, code: &str) -> String {
    format!("{}@{}", sym.to_string(), code)
}
