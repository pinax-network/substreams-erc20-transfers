use crate::abi::{self};
use crate::pb::erc20::types::v1::{TransferEvents, TransferEvent};
use abi::erc20::events::Transfer;
use substreams::errors::Error;
use substreams::Hex;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;


#[substreams::handlers::map]
fn index_transfers(transfers: TransferEvents) -> Result<Keys, Error> {
    Ok(match transfers.transfers.is_empty() {
        true => Keys::default(),
        false => Keys {
            keys: vec!["transfers".to_string()]
        },
    })
}



#[substreams::handlers::map]
pub fn map_transfers(block: Block) -> Result<TransferEvents, Error> {
    let  transfers = map_events(&block);

    Ok(TransferEvents {
        transfers
    })
}

pub fn map_events(block: &Block) -> Vec<TransferEvent> {
    let mut transfers = vec![];

    for log in block.logs() {
        
        if let Some(transfer) = Transfer::match_and_decode(log.log) {
            transfers.push(decode_transfer(transfer, log));
            continue;
        }

        // no data
    }

    return transfers;
}

fn decode_transfer(event: Transfer, log: LogView) -> TransferEvent {
    TransferEvent {
        // contract address
        address: Hex::encode(log.address()),

        // event payload
        from: Hex::encode(event.from),
        to: Hex::encode(event.to),
        value: event.value.to_string(),

        // trace information
        transaction: Hex::encode(&log.receipt.transaction.hash),
        block_index: log.log.block_index.into(),
    }
}

