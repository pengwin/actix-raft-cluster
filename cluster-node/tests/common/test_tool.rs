use std::sync::Arc;

use std::time::Duration;
use tokio::select;
use tokio::task::JoinHandle;
use tokio::time::sleep;

use super::test_node::start;
use cluster_node::{NodeConfig, NodeError, ClusterNode};
use node_actor::{Metrics, NodeMetrics, RemoteNodeActorAddr};
use tokio::sync::oneshot::channel;

pub struct NodeGuard {
    node: Arc<ClusterNode>
}

impl Drop for NodeGuard {
    fn drop(&mut self) {
        tracing::info!("Dropping");
        (*self.node).stop_sync().expect("Unable to stop node");
        tracing::info!("Dropped");
    }
}

pub struct TestTool {}

impl TestTool {
    pub async fn start(cfg: Arc<NodeConfig>) -> (NodeGuard, JoinHandle<Result<(), NodeError>>) {
        let (tx, rx) = channel();
        let h = tokio::task::spawn_blocking(move || {
            std::thread::spawn(move || start(cfg, tx)).join().expect("Unable to join thread")
        });
        
        let node = rx.await.expect("Unable to receive node");
        (NodeGuard{node}, h)
    }

    pub async fn get_metrics(cfg: &NodeConfig) -> Result<NodeMetrics, String> {
        let addr = cfg.this_node.addr();
        let node_id = cfg.this_node.node_id;
        let node = RemoteNodeActorAddr::new(node_id, addr);

        let send_res = node
            .send(&Metrics {})
            .await
            .map_err(|e| format!("{:?}", e))?;

        send_res.map_err(|e| format!("{:?}", e))
    }

    pub async fn wait_for_activation(cfg: Arc<NodeConfig>) -> Result<NodeMetrics, String> {
        let timeout = Duration::from_millis(10);
        let max_reties = 10u8;
        let mut last_error: String = String::default();

        for _ in 0..max_reties {
            let res = select! {
                val = Self::get_metrics(&*cfg) => val,
                _ = sleep(timeout) => {
                    Err("Timeout".to_string())
                }
            };

            match res {
                Ok(metrics) => {
                    return Ok(metrics);
                }
                Err(e) => {
                    last_error = e;
                }
            }
        }

        Err(format!("Max retries reached {}", last_error))
    }
}
