use std::sync::Arc;

use cluster_node::{ClusterNode, NodeConfig, NodeError};
use tokio::sync::oneshot::Sender;

pub fn start(cfg: &NodeConfig, tx: Sender<Arc<ClusterNode>>) -> Result<(), NodeError> {
    let sys = actix_web::rt::System::new();
    sys.block_on(async move {
        let node = ClusterNode::new(cfg)?;
        let arc_node = Arc::new(node);
        let _ = tx.send(arc_node.clone());
        arc_node.run().await
    })
}
