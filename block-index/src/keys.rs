pub fn db_ops_table_key(code: &str, scope: &str, table_name: &str, primary_key: &str) -> String {
    format!("{}:{}:{}:{}", code, scope, table_name, primary_key)
}
