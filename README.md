# miner-btc

Experiments in Bitcoin mining

## Peer discovery

- DNS seeds and fallback addresses are hardcoded into the [Bitcoin Core](https://github.com/bitcoin/bitcoin)
  - For mainnet (testnet) refer to `CMainParams` (`CTestNetParams`) of [src/chainparams.cpp](https://github.com/bitcoin/bitcoin/block/master/src/chainparams.cpp)
- Validated peers should be stored locally for subsequent startups
- Ref: [Bitcoin > Developer Guides > P2P Network](https://developer.bitcoin.org/devguide/p2p_network.html)
