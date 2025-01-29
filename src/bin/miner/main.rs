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
        .arg(clap::arg!(-m --mainnet))
        .arg(clap::arg!(-t --testnet))
        .group(
            clap::ArgGroup::new("network")
                .required(true)
                .args(["mainnet", "testnet"]),
        )
        .arg(clap::arg!(<addr>).required(true))
        // FIXME Optional argument for socks proxy support
        .get_matches();

    let addr = {
        let addr = matches.get_one::<String>("addr").unwrap().clone();

        // Ref: https://en.bitcoin.it/wiki/List_of_address_prefixes
        let addr_prefix = addr.chars().next().unwrap();
        if matches.get_flag("mainnet") {
            assert!(addr_prefix == '1');
        } else {
            assert!(addr_prefix == 'm' || addr_prefix == 'n');
        }

        log::info!("reward address: {}", addr);
        addr
    };

    let nodes = if matches.get_flag("mainnet") {
        log::info!("connecting to mainnet");
        p2p::mainnet_nodes()
    } else {
        log::info!("connecting to testnet");
        p2p::testnet_nodes()
    };

    tokio::spawn(net::net_mgr_task(addr, nodes)).await?;

    Ok(())
}
