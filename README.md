# ERC-20 Substreams

> Substreams for ERC-20 tokens.

## Quickstart

```
$ gh repo clone pinax-network/substreams-erc20-transfers-approuvals
$ cd substreams-erc20
$ make
$ make gui
```

## Releases

- https://github.com/pinax-network/substreams-erc20-transfers-approuvals/releases

## References

- [Ethereum Docs: ERC-20 Token Standard](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)
- [EIPS: ERC-20 Token Standard ](https://eips.ethereum.org/EIPS/eip-20)
- [OpenZeppelin implementation](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/9b3710465583284b8c4c5d2245749246bb2e0094/contracts/token/ERC20/ERC20.sol)
- [ConsenSys implementation](https://github.com/ConsenSys/Tokens/blob/fdf687c69d998266a95f15216b1955a4965a0a6d/contracts/eip20/EIP20.sol)

### Events

| Event                                                          | Description                                                                         |
| -------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| [`Transfer`](https://eips.ethereum.org/EIPS/eip-20#transfer-1) | MUST trigger when tokens are transferred, including zero value transfers.           |
| [`Approval`](https://eips.ethereum.org/EIPS/eip-20#approval)   | MUST trigger on any successful call to `approve(address _spender, uint256 _value)`. |

### Mermaid Graph

```mermaid
graph TD;
  map_block[map: map_block];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_block;
  graph_out[map: graph_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> graph_out;
  map_block --> graph_out;

```

Here is a quick link to see the graph:

https://mermaid.live/edit#pako:eJx0j0FqAzEMRa9itJ6YtN25uzY3aHd1CIqtZEKj2NhSIIS5e8EDM22ZLL_-5z10h5AigYNjwdybz82rvxjDmHf7cwrfX4zZzXHb2nqwJD0VUrZyy2Svz_atrWvSEsg9XmzNauV1vX6hGdqYTb9LKqNxipOx6r5KIeRqr0_2_b9uof7lmnB_v1seQAdMhfEUwcHdg_TE5MF5iHRAPYuHATpAlfRxuwRwUpQ60BxRaHPCY0Eej8NPAAAA__89CXqq

### Modules

```yaml
Package name: erc20
Version: v0.3.0
Doc: ERC-20
Modules:
----
Name: map_block
Initial block: 0
Kind: map
Output Type: proto:erc20.types.v1.Block
Hash: d2e09cd303cbcba7596ecb53736b823f480e6450
Doc: Extracts 'Approval' & 'Transfer' events from the block

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: 66ff1796c3eb93e9ab07c0ce6417993e84f7859b

```
