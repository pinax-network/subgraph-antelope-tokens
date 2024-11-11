use std::collections::HashMap;
use substreams::errors::Error;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_entity_change::pb::entity::EntityChanges;

use crate::eosio_token::Events;

#[substreams::handlers::map]
pub fn ch_out(map_events: Events) -> Result<DatabaseChanges, Error> {
    let mut tables = DatabaseChanges::default();

    for account in map_events.balance_changes {
        let keys = HashMap::from([
            ("account".to_string(), account.account.to_string()),
            ("block_num".to_string(), account.block_num.to_string()),
            ("trx_id".to_string(), account.trx_id),
            ("index".to_string(), account.index.to_string()),
        ]);

        tables
            .push_change_composite("balance_change_events", keys, 0, table_change::Operation::Create)
            .change("contract", ("", account.contract.to_string().as_str()))
            .change("symcode", ("", account.symcode.to_string().as_str()))
            .change("balance", ("", account.balance.to_string().as_str()))
            .change("balance_delta", ("", account.balance_delta.to_string().as_str()))
            .change("precision", ("", account.precision.to_string().as_str()))
            .change("amount", ("", account.amount.to_string().as_str()))
            .change("value", ("", account.value.to_string().as_str()))
            .change("timestamp", ("", account.timestamp.unwrap().to_string().as_str()));
    }

    for stat in map_events.supply_changes {
        let keys = HashMap::from([
            ("contract".to_string(), stat.contract.to_string()),
            ("block_num".to_string(), stat.block_num.to_string()),
            ("trx_id".to_string(), stat.trx_id),
            ("index".to_string(), stat.index.to_string()),
        ]);

        tables
            .push_change_composite("supply_change_events", keys, 0, table_change::Operation::Create)
            .change("symcode", ("", stat.symcode.to_string().as_str()))
            .change("issuer", ("", stat.issuer.to_string().as_str()))
            .change("max_supply", ("", stat.max_supply.to_string().as_str()))
            .change("supply", ("", stat.supply.to_string().as_str()))
            .change("supply_delta", ("", stat.supply_delta.to_string().as_str()))
            .change("precision", ("", stat.precision.to_string().as_str()))
            .change("amount", ("", stat.amount.to_string().as_str()))
            .change("value", ("", stat.value.to_string().as_str()))
            .change("timestamp", ("", stat.timestamp.unwrap().to_string().as_str()));
    }

    for transfer in map_events.transfers {
        let keys = HashMap::from([("trx_id".to_string(), transfer.trx_id), ("index".to_string(), transfer.index.to_string())]);

        tables
            .push_change_composite("transfer_events", keys, 0, table_change::Operation::Create)
            .change("contract", ("", transfer.contract.to_string().as_str()))
            .change("symcode", ("", transfer.symcode.to_string().as_str()))
            .change("from", ("", transfer.from.to_string().as_str()))
            .change("to", ("", transfer.to.to_string().as_str()))
            .change("memo", ("", transfer.memo.to_string().as_str()))
            .change("quantity", ("", transfer.quantity.to_string().as_str()))
            .change("amount", ("", transfer.amount.to_string().as_str()))
            .change("precision", ("", transfer.precision.to_string().as_str()))
            .change("value", ("", transfer.value.to_string().as_str()))
            .change("block_num", ("", transfer.block_num.to_string().as_str()))
            .change("timestamp", ("", transfer.timestamp.unwrap().to_string().as_str()));
    }

    for issue in map_events.issues {
        let keys = HashMap::from([
            ("contract".to_string(), issue.contract),
            ("symcode".to_string(), issue.symcode),
            ("to".to_string(), issue.to),
            ("amount".to_string(), issue.amount.to_string()),
            ("trx_id".to_string(), issue.trx_id),
            ("action_ordinal".to_string(), issue.action_ordinal.to_string()),
        ]);

        tables
            .push_change_composite("issue_events", keys, 0, table_change::Operation::Create)
            .change("quantity", ("", issue.quantity.to_string().as_str()))
            .change("memo", ("", issue.memo.to_string().as_str()))
            .change("precision", ("", issue.precision.to_string().as_str()))
            .change("value", ("", issue.value.to_string().as_str()))
            .change("block_num", ("", issue.block_num.to_string().as_str()))
            .change("timestamp", ("", issue.timestamp.unwrap().to_string().as_str()));
    }

    for retire in map_events.retires {
        let keys = HashMap::from([
            ("contract".to_string(), retire.contract),
            ("symcode".to_string(), retire.symcode),
            ("amount".to_string(), retire.amount.to_string()),
            ("trx_id".to_string(), retire.trx_id),
            ("index".to_string(), retire.index.to_string()),
        ]);

        tables
            .push_change_composite("retire_events", keys, 0, table_change::Operation::Create)
            .change("quantity", ("", retire.quantity.to_string().as_str()))
            .change("memo", ("", retire.memo.to_string().as_str()))
            .change("precision", ("", retire.precision.to_string().as_str()))
            .change("value", ("", retire.value.to_string().as_str()))
            .change("block_num", ("", retire.block_num.to_string().as_str()))
            .change("timestamp", ("", retire.timestamp.unwrap().to_string().as_str()));
    }

    for create in map_events.creates {
        let keys = HashMap::from([
            ("contract".to_string(), create.contract),
            ("symcode".to_string(), create.symcode),
            ("trx_id".to_string(), create.trx_id),
            ("index".to_string(), create.index.to_string()),
        ]);

        tables
            .push_change_composite("create_events", keys, 0, table_change::Operation::Create)
            .change("issuer", ("", create.issuer.to_string().as_str()))
            .change("maximum_supply", ("", create.maximum_supply.to_string().as_str()))
            .change("precision", ("", create.precision.to_string().as_str()))
            .change("amount", ("", create.amount.to_string().as_str()))
            .change("value", ("", create.value.to_string().as_str()))
            .change("block_num", ("", create.block_num.to_string().as_str()))
            .change("timestamp", ("", create.timestamp.unwrap().to_string().as_str()));
    }

    Ok(tables)
}
