
use crate::pb::erc20::types::v1::Block as Erc20Block;
use substreams::errors::Error;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;


#[substreams::handlers::map]
pub fn graph_out(block: Erc20Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for event in block.transfers {
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

    for event in block.approvals {
        let id = format!("{}-{}", event.block_index, event.transaction);

        tables
            .create_row("Approvals", id)
            // contract address
            .set("address", event.address)
            // event payload
            .set("owner", event.owner)
            .set("spender", event.spender)
            .set("value", event.value)
            // trace information
            .set("transaction", event.transaction);
    }

    Ok(tables.to_entity_changes())
}
