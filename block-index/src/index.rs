use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_antelope::Block;

// filter blocks that DO NOT match "accounts" or "stat" tables
#[substreams::handlers::map]
fn index_blocks(block: Block) -> Result<Keys, substreams::errors::Error> {
    for transaction in block.transaction_traces() {
        for db_op in transaction.db_ops.iter() {
            if db_op.table_name == "accounts" || db_op.table_name == "stat" {
                return Ok(Keys { keys: vec!["blockIndex".to_string()] });
            }
        }
    }
    Ok(Keys::default())
}
