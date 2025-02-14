use crate::p2p::Nodes;

pub async fn net_mgr_task(/* FIXME Temp name */ _addr: String, nodes: Nodes) {
    // FIXME Define an async network manager task
    //       which launches network worker tasks and distributes SocketAddrs
    let mut node_count = 0;
    for _ in nodes {
        node_count += 1;
    }
    log::info!("{} nodes found", node_count);

    // FIXME Send ping if connection inactive for 30 minutes, with reasonble timeout for pong (not the default 20 mins)
    // FIXME Respond appropriately to pings
}
