use crate::p2p::Nodes;

pub async fn net_mgr_task(/* FIXME Temp name */ _addr: String, nodes: Nodes) {
    // FIXME STOPPED Define an async network manager task which launches network worker tasks and distributes SocketAddrs
    //               Implement the behavior defined by the two sequence diagrams in the README.md
    //               and additionally (not in diagrams) send pings for inactive connections (30 mins) and respond to incoming pings
    let mut node_count = 0;
    for _ in nodes {
        node_count += 1;
    }
    log::info!("{} nodes found", node_count);
}
