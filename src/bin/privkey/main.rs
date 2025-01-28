use miner_btc::ecdsa::PrivKey;

fn main() {
    let matches = clap::command!()
        .arg(clap::arg!(-m - -mainnet))
        .arg(clap::arg!(-t - -testnet))
        .group(
            clap::ArgGroup::new("network")
                .required(true)
                .args(["mainnet", "testnet"]),
        )
        .get_matches();

    let (privkey, pubkey) = PrivKey::rand_bitcoin();
    println!("{}", privkey);
    println!("{}", pubkey.base58_addr(matches.get_flag("mainnet")));
}
