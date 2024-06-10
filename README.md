# ERC-20 Substreams

> Substreams for ERC-20 tokens.

## Quickstart

```
$ gh repo clone pinax-network/substreams-erc20-transfers
$ cd substreams-erc20
$ make
$ make gui
```

## Releases

- https://github.com/pinax-network/substreams-erc20-transfers/releases

## References

- [Ethereum Docs: ERC-20 Token Standard](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)
- [EIPS: ERC-20 Token Standard ](https://eips.ethereum.org/EIPS/eip-20)
- [OpenZeppelin implementation](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/9b3710465583284b8c4c5d2245749246bb2e0094/contracts/token/ERC20/ERC20.sol)
- [ConsenSys implementation](https://github.com/ConsenSys/Tokens/blob/fdf687c69d998266a95f15216b1955a4965a0a6d/contracts/eip20/EIP20.sol)

### Events

| Event                                                          | Description                                                               |
| -------------------------------------------------------------- | ------------------------------------------------------------------------- |
| [`Transfer`](https://eips.ethereum.org/EIPS/eip-20#transfer-1) | MUST trigger when tokens are transferred, including zero value transfers. |

### Mermaid Graph

```mermaid
graph TD;
  map_transfers[map: map_transfers];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_transfers;
  map_transfers --> index_transfers;
  graph_out[map: graph_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> graph_out;
  map_transfers --> graph_out;
  db_out[map: db_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> db_out;
  map_transfers --> db_out;

```

Here is a quick link to see the graph:

https://mermaid.live/edit#pako:eJx0js9qwzAMh1_F6Jyast3c2-gbbLc6FGErTVn9B1kalJB3HySQEUaP-vHx6ZsglEjg4MZYR_N1PvlsTMJ6FcbcBuJ2SVjdfuoXqg2WZCQmTVaelezPm_14lPB9aUU5kHtN9OZw8Ho8vtNevHiXlGtRWT9vZ_-_7U-zUSefoYNEnPAewcHkQUZK5MF5iDSgPsTDDB2gSvl85gBOWKkDrRGFzne8MaZ1nH8DAAD__-8lZkw

### Modules

```yaml
Name: map_transfers
Initial block: 0
Kind: map
Input: source: sf.ethereum.type.v2.Block
Output Type: proto:erc20.types.v1.TransferEvents
Hash: 535bceb547b75a87d5fa50198a4c75aa53dbc79d
Doc:  Extracts 'Transfer' events from the block

Name: index_transfers
Initial block: 0
Kind: index
Input: map: map_transfers
Output Type: proto:sf.substreams.index.v1.Keys
Hash: df6b7d955f808b4d651f2788ab23870f0c75b958

Name: graph_out
Initial block: 0
Kind: map
Input: source: sf.substreams.v1.Clock
Input: map: map_transfers
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: b78854193ba93ab43477a658f706b0fc63b57996

Name: db_out
Initial block: 0
Kind: map
Input: source: sf.substreams.v1.Clock
Input: map: map_transfers
Block Filter: (using *index_transfers*): `&{inscription}`
Output Type: proto:sf.substreams.sink.database.v1.DatabaseChanges
Hash: e2b5ade6d06a300688f1545793bc78d661c88bb6

```
