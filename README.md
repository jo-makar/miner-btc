# miner-btc

Experiments in Bitcoin mining

## Peer discovery

- DNS seeds and fallback addresses are hardcoded into [Bitcoin Core](https://github.com/bitcoin/bitcoin)
  - For mainnet (testnet) refer to `CMainParams` (`CTestNetParams`) of [src/chainparams.cpp](https://github.com/bitcoin/bitcoin/block/master/src/chainparams.cpp)
  - Or preferably [contrib/seeds/nodes_main.txt](https://github.com/bitcoin/bitcoin/block/master/contrib/seeds/nodes_main.txt) and [contrib/seeds/nodes_test.txt](https://github.com/bitcoin/bitcoin/block/master/contrib/seeds/nodes_test.txt)
- Validated peers should be stored locally for subsequent startups
- Ref: [Bitcoin > Developer Guides > P2P Network](https://developer.bitcoin.org/devguide/p2p_network.html)

## Proxy use

- Install tsocks (eg `apt-get install tsocks`)
  - May need to install from source for proxied DNS lookup support, ref `tsocks(8)`
- Configure tsocks 
  - `mv /etc/tsocks.conf /etc/tsocks.conf.orig`
  - ```
    cat <<EOF >/etc/tsocks.conf
    server = 127.0.0.1
    server_type = 5
    server_port = 1080
    EOF
    ```
- Launch the SOCKS5 proxy (eg `ssh -nNT -D 127.0.0.1:1080 <host>`)
- `tsocks cargo run [--bin <binary>] [-- <arguments>]`