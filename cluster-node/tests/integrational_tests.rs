mod common;

use std::sync::Arc;
use tokio::select;
use tracing::{info, info_span};

use cluster_node::{NodeConfig, RemoteNodeConfig, RemoteNodeConfigProtocol};

use crate::common::{setup_tracing, TestTool};

#[test]
fn leader_metrics() -> Result<(), String> {
    setup_tracing();
    let span = info_span!("test_main");
    let _enter = span.enter();

    let cfg = Arc::new(NodeConfig {
        sever_workers_number: 1,
        cluster_name: "test_cluster".to_owned(),
        this_node: RemoteNodeConfig {
            node_id: 1,
            host: "127.0.0.1".to_string(),
            port: 8080,
            protocol: RemoteNodeConfigProtocol::Http,
        },
    });

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .worker_threads(2)
        .thread_name("test_thread")
        .build()
        .map_err(|e| format!("Start rt {:?}", e))?;

    let rt_handle = rt.handle();

    rt.block_on(async move {
        let (_node_guard, h) = TestTool::start(cfg.clone()).await;
        select! {
            v = h => match v {
                Ok(_) => {
                    Err("Node stopped".to_string())
                },
                Err(e) => {
                    Err(format!("Node error {:?}", e))
                }
            },
            v = rt_handle.spawn(TestTool::wait_for_activation(cfg.clone())) => match v {
                Ok(wait_res) => {
                    let m = wait_res?;

                    assert_eq!(m.nodes.len(), 1);

                    info!("Done");
                    Ok(())
                },
                Err(e) => {
                    Err(format!("Activation error {:?}", e))
                }
            }
        }
    })
}

#[test]
fn attach() -> Result<(), String> {
    setup_tracing();
    let span = info_span!("test_main");
    let _enter = span.enter();

    let cfg_leader = Arc::new(NodeConfig {
        sever_workers_number: 1,
        cluster_name: "test_cluster".to_owned(),
        this_node: RemoteNodeConfig {
            node_id: 1,
            host: "127.0.0.1".to_string(),
            port: 8080,
            protocol: RemoteNodeConfigProtocol::Http,
        },
    });

    let cfg_follower = Arc::new(NodeConfig {
        sever_workers_number: 1,
        cluster_name: "test_cluster".to_owned(),
        this_node: RemoteNodeConfig {
            node_id: 2,
            host: "127.0.0.1".to_string(),
            port: 8081,
            protocol: RemoteNodeConfigProtocol::Http,
        },
    });

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .worker_threads(2)
        .thread_name("test_thread")
        .build()
        .map_err(|e| format!("Start rt {:?}", e))?;

    let rt_handle = rt.handle();

    rt.block_on(async move {
        let (_leader_guard, leader) = TestTool::start(cfg_leader.clone()).await;
        let (_follower_guard, follower) = TestTool::start(cfg_follower.clone()).await;

        let test_body = async move {
            let _ = TestTool::wait_for_activation(cfg_leader.clone()).await?;
            match TestTool::wait_for_activation(cfg_follower.clone()).await {
                Ok(m_follower) => {
                    let m_leader = TestTool::get_metrics(cfg_leader.clone()).await?;

                    assert_eq!(m_leader.nodes.len(), 2, "assert leader nodes");
                    assert_eq!(m_follower.nodes.len(), 2, "assert follower nodes");

                    Ok(())
                }
                Err(e) => Err(e),
            }
        };

        select! {
            v = leader => match v {
                Ok(_) => {
                    Err("leader stopped".to_string())
                },
                Err(e) => {
                    Err(format!("leader error {:?}", e))
                }
            },
            v = follower => match v {
                Ok(_) => {
                    Err("follower stopped".to_string())
                },
                Err(e) => {
                    Err(format!("follower error {:?}", e))
                }
            },
            v = rt_handle.spawn(test_body) => match v {
                Ok(wait_res) => {
                    let _ = wait_res?;

                    info!("Done");
                    Ok(())
                },
                Err(e) => {
                    Err(format!("Activation error {:?}", e))
                }
            }
        }
    })
}
