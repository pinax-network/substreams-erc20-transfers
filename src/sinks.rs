use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_database_change::pb::database::{DatabaseChanges, table_change::Operation};
use crate::pb::erc20::types::v1::TransferEvents;


#[substreams::handlers::map]
pub fn graph_out(clock: Clock,transfers: TransferEvents) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();


    let block = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

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
            .set("transaction", event.transaction)
            .set("block_number", block.clone())
            .set("timestamp", timestamp.clone());
    }
    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
fn db_out(clock: Clock,transfers: TransferEvents) -> Result<DatabaseChanges, Error> {

    // Initialize Database Changes container
    let mut database_changes: DatabaseChanges = Default::default();

    let block = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for event in transfers.transfers {
        let id = format!("{}-{}", event.block_index, event.transaction);

    // Create row 
    database_changes.push_change("Transfers", id, 0, Operation::Create)
        .change("address", (None, event.address))
        .change("from", (None, event.from))
        .change("to", (None, event.to))
        .change("value", (None, event.value))
        .change("transaction", (None, event.transaction))
        .change("block_number", (None,block.clone() ))
        .change("timestamp", (None, timestamp.clone()));
    }

    Ok(database_changes)
}


