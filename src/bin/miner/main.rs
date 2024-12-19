use std::io;
use std::io::Write;

use miner_btc::net;
use miner_btc::p2p;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.level(),
                record.args(),
            )
        })
        .filter(None, log::LevelFilter::Info)
        .init();

    let matches = clap::command!()
        .arg(clap::arg!(-m - -mainnet))
        .arg(clap::arg!(-t - -testnet))
        .group(
            clap::ArgGroup::new("network")
                .required(true)
                .args(["mainnet", "testnet"]),
        )
        // FIXME STOPPED Positional argument for reward address which should be validated
        .get_matches();

    let nodes = if matches.get_flag("mainnet") {
        log::info!("connecting to mainnet");
        p2p::mainnet_nodes()
    } else {
        log::info!("connecting to testnet");
        p2p::testnet_nodes()
    };

    tokio::spawn(net::net_mgr_task(nodes)).await?;

    Ok(())
}
