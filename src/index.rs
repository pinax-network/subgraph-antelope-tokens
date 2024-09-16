use substreams_antelope::pb::DbOp;

// i.e. https://docs.dfuse.eosnation.io/eosio/public-apis/reference/search/terms/
pub fn collect_db_op_keys(db_op: &DbOp) -> Vec<String> {
    let mut keys = Vec::new();

    // Skip `eosio:*` tables
    // these tables force every block to be indexed
    let skip_tables = vec!["global", "global2", "global3", "global4", "blockinfo", "producers"];
    if db_op.code == "eosio" && skip_tables.contains(&db_op.table_name.as_str()) {
        return vec![];
    }

    // db.table:accounts/swap.defi account:eosio.token
    keys.extend(vec![
        format!("code:{}", db_op.code),
        format!("db.table:{}", db_op.table_name),
        format!("db.table:{}/{}", db_op.table_name, db_op.scope),
    ]);

    keys
}
