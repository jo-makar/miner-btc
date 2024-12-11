use rand::Rng;

use std::collections::HashSet;
use std::net::{SocketAddr, ToSocketAddrs};

use super::mainnet;
use super::testnet;

pub fn mainnet_nodes() -> Nodes {
    Nodes::new(
        "mainnet.json".to_string(),
        SeedNodes::new(mainnet::SEEDS, &mainnet::FIXED_NODES, 8333),
    )
}

pub fn testnet_nodes() -> Nodes {
    Nodes::new(
        "testnet.json".to_string(),
        SeedNodes::new(testnet::SEEDS, &testnet::FIXED_NODES, 18333),
    )
}

pub struct Nodes {
    _path: String, // FIXME Temp name
    known_nodes: Vec<SocketAddr>,
    relayed_nodes: Vec<SocketAddr>,
    seed_nodes: SeedNodes,
    seen_nodes: HashSet<SocketAddr>,
    _tested_known_nodes: Vec<SocketAddr>, // FIXME Temp name
}

impl Nodes {
    fn new(path: String, seed_nodes: SeedNodes) -> Nodes {
        // FIXME STOPPED Populate {known,relayed}_nodes from persisted json stored at path

        Nodes {
            _path: path,
            known_nodes: Vec::new(),
            relayed_nodes: Vec::new(),
            seed_nodes,
            seen_nodes: HashSet::new(),
            _tested_known_nodes: Vec::new(),
        }
    }

    // FIXME STOPPED Add Drop behavior to persist json at path (or use an explicit method?)
    // FIXME Add a method to add to relayed_nodes
    // FIXME Add a method to add to tested_known_nodes
}

impl Iterator for Nodes {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_addr = || -> Option<Self::Item> {
            if !self.known_nodes.is_empty() {
                return self.known_nodes.pop();
            }

            if !self.relayed_nodes.is_empty() {
                return self.relayed_nodes.pop();
            }

            self.seed_nodes.next()
        };

        while let Some(node) = next_addr() {
            if self.seen_nodes.insert(node) {
                return Some(node);
            }
        }

        None
    }
}

pub struct SeedNodes {
    seeds: Vec<String>,
    seed_nodes: Vec<SocketAddr>,
    fixed_nodes: Vec<SocketAddr>,
    port: u16,
    seen_nodes: HashSet<SocketAddr>,
    duplicates: usize,
}

impl SeedNodes {
    fn new(seeds: &[&str], fixed_nodes: &[SocketAddr], port: u16) -> SeedNodes {
        SeedNodes {
            seeds: {
                let mut v: Vec<String> = seeds.iter().map(|&s| String::from(s)).collect();
                shuffle(&mut v);
                v
            },
            seed_nodes: Vec::new(),
            fixed_nodes: {
                let mut v = fixed_nodes.to_vec();
                shuffle(&mut v);
                v
            },
            port,
            seen_nodes: HashSet::new(),
            duplicates: 0,
        }
    }
}

impl Iterator for SeedNodes {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_addr = || -> Option<Self::Item> {
            if !self.seed_nodes.is_empty() {
                return self.seed_nodes.pop();
            }

            while let Some(mut seed) = self.seeds.pop() {
                seed += format!(":{}", self.port).as_str();

                match seed.to_socket_addrs() {
                    Ok(addrs) => {
                        self.seed_nodes = addrs.collect();
                        shuffle(&mut self.seed_nodes);

                        log::info!("{} resolved into {} addresses", seed, self.seed_nodes.len());
                        if !self.seed_nodes.is_empty() {
                            return self.seed_nodes.pop();
                        }
                    }
                    Err(err) => log::error!("failed to resolve {}: {}", seed, err),
                }
            }

            if !self.fixed_nodes.is_empty() {
                return self.fixed_nodes.pop();
            }

            None
        };

        while let Some(node) = next_addr() {
            if self.seen_nodes.insert(node) {
                return Some(node);
            }
            self.duplicates += 1;
        }

        log::info!("{} duplicate addresses found", self.duplicates);
        None
    }
}

fn shuffle<T>(v: &mut Vec<T>) {
    let mut rng = rand::thread_rng();

    let n = v.len();
    for i in 0..(n - 1) {
        let j = rng.gen_range(i..n);
        v.swap(i, j);
    }
}
