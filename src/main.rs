use std::io::Write;

mod p2p;

fn main() {
    env_logger::Builder::new()
        .format(
            |buf, record| writeln!(
                              buf,
                              "{} [{}] {}",
                              chrono::Local::now().format("%H:%M:%S"),
                              record.level(),
                              record.args(),
                          )
        )
        .filter(None, log::LevelFilter::Info)
        .init();

    // FIXME STOPPED Command line flag to specify mainnet / testnet
    let nodes = if true { p2p::mainnet_nodes() } else { p2p::testnet_nodes() };
    let mut node_count = 0;
    for _ in nodes {
        node_count += 1;
    }
    log::info!("{} nodes found", node_count);
}
