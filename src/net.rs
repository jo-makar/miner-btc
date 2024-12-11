use crate::p2p::SeedNodes;

pub async fn net_mgr_task(nodes: SeedNodes) {
    // FIXME Define an async network manager task
    //       which launches network worker tasks and distributes SocketAddrs
    let mut node_count = 0;
    for _ in nodes { node_count += 1; }
    log::info!("{} nodes found", node_count);
}