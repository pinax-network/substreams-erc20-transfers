use substreams::{errors::Error, log};
use substreams::pb::substreams::Clock;
use substreams_database_change::change;
use substreams_database_change::pb::database::{DatabaseChanges, table_change::Operation};
use crate::pb::erc20::types::v1::TransferEvents;

#[substreams::handlers::map]
fn db_out(clock: Clock,transfers: TransferEvents) -> Result<DatabaseChanges, Error> {

    // Initialize Database Changes container
    let mut database_changes: DatabaseChanges = Default::default();

    let block = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for event in transfers.transfers {
    let id = format!("{}-{}", event.tx_id.clone(),event.index.clone());
    // Create row 
    database_changes.push_change("transfers", id, 0, Operation::Create)
        .change("contract", (None, event.contract))
        .change("from", (None, event.from))
        .change("to", (None, event.to))
        .change("value", (None, event.value))
        .change("tx_id", (None, event.tx_id))
        .change("action_index", (None, event.index))
        .change("block_num", (None,block.clone() ))
        .change("timestamp", (None, timestamp.clone()));
    }

    Ok(database_changes)
}


