use substreams::errors::Error;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

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
