use actix::{Arbiter, ArbiterHandle, System};
use actix_web::dev::Server;
use tracing::{info_span, Instrument};

use actor_registry::NodesRegistry;

use crate::web_server::create_server;
use crate::config::NodeConfig;

use super::RegistryCollection;
use super::attach_to_leader::attach_to_leader;
use crate::node::error::NodeError;
use std::sync::Arc;

/// Cluster Node
/// Starts server, actor system and creates nodes registry
pub struct ClusterNode {
    srv: Server,
    system: System,
    main_arbiter: ArbiterHandle,
    registry: RegistryCollection,
}

impl ClusterNode {
    /// Creates new node with provided config
    #[tracing::instrument]
    pub fn new(config: Arc<NodeConfig>) -> Result<ClusterNode, NodeError> {
        if !actix_rt::System::is_registered() {
            return Err(NodeError::ThreadDoesntHaveSystem)
        }
        
        let server_config = config.server_config();

        let nodes = NodesRegistry::new(
            config.this_node.node_id,
            &config.this_node.addr());

        let registry = RegistryCollection::new(nodes.clone());

        let srv = create_server(&server_config, registry.clone())?;
        
        let main_arbiter= Arbiter::new();
        
        main_arbiter.spawn(async move {
            let thread_cfg = config.clone();
            let leader_node = &thread_cfg.leader_node;
            let this_node = &thread_cfg.this_node;
            if let Some(leader_node) = leader_node {
                let r = attach_to_leader(nodes, &leader_node, &this_node).await;
                match r {
                    Ok(b) => match b {
                        true => tracing::info!("Node attached"),
                        false => {}
                    },
                    Err(e) => {
                        tracing::error!("Failed to attach to node {:?}", e)
                    }
                }
            }
        });
        
        let system = System::current();
        
        Ok(ClusterNode{
            system,
            main_arbiter: main_arbiter.handle(),
            srv,
            registry,
        })
    }

    /// Runs node
    #[tracing::instrument(skip(self))]
    pub async fn run(&self) -> Result<(), NodeError> {
        let srv = self.srv.clone();
        srv.await?;
        
        Ok(())
    }

    /// Stops node asynchronously
    #[allow(dead_code)]
    #[tracing::instrument(skip(self))]
    pub async fn stop(&self) -> Result<(), NodeError> {
        let srv = self.srv.clone();
        
        srv.stop(true).instrument(info_span!("Stop server")).await;
        
        let arb = self.main_arbiter.clone();
        let span = info_span!("Stop main arbiter");
        let e = span.enter();
        arb.stop();
        drop(e);
        
        let reg = self.registry.clone();
        let span = info_span!("Stop registry");
        let e = span.enter();
        reg.stop();
        drop(e);
        
        Ok(())
    }

    /// Stops node synchronously
    #[allow(dead_code)]
    #[tracing::instrument(skip(self))]
    pub fn stop_sync(&self) -> Result<(), NodeError> {
        let arb = self.system.arbiter();
        let span = info_span!("Stop main arbiter");
        let e = span.enter();
        let srv = self.srv.clone();
        arb.spawn(async move {
            srv.stop(true).instrument(info_span!("Stop server")).await;
        });
        drop(e);

        let reg = self.registry.clone();
        let span = info_span!("Stop registry");
        let e = span.enter();
        reg.stop();
        drop(e);
        
        Ok(())
    }
}
