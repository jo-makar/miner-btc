mod p2p;

fn main() {
    println!("{} {}", p2p::mainnet::SEEDS.len(), p2p::mainnet::FIXED_NODES.len());
    println!("{} {}", p2p::testnet::SEEDS.len(), p2p::testnet::FIXED_NODES.len());
}
