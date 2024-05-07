use substreams::errors::Error;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_database_change::pb::database::{DatabaseChanges, table_change::Operation};
use crate::pb::erc20::types::v1::TransferEvents;


#[substreams::handlers::map]
pub fn graph_out(transfers: TransferEvents) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for event in transfers.transfers {
        let id = format!("{}-{}", event.block_index, event.transaction);

        tables
            .create_row("Transfers", id)
            // contract address
            .set("address", event.address)
            // event payload
            .set("from", event.from)
            .set("to", event.to)
            .set("value", event.value)
            // trace information
            .set("transaction", event.transaction);
    }
    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn db_out(transfers: TransferEvents) -> Result<DatabaseChanges, Error> {

    // Initialize Database Changes container
    let mut database_changes: DatabaseChanges = Default::default();

    
    for event in transfers.transfers {
        let id = format!("{}-{}", event.block_index, event.transaction);

    // Create row 
    database_changes.push_change("Transfer", id, 0, Operation::Create)
        .change("address", (None, event.address))
        .change("from", (None, event.from))
        .change("to", (None, event.to))
        .change("value", (None, event.value))
        .change("transaction", (None, event.transaction));
    }

    Ok(database_changes)
}


