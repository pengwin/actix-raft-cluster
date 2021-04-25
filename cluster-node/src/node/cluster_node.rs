use std::sync::Arc;
use actix::{ArbiterHandle,  Arbiter};
use actix_web::dev::Server;
use tracing::{info_span, Instrument};

use actor_registry::NodesRegistry;

use crate::web_server::create_server;
use crate::config::NodeConfig;

use super::RegistryCollection;
use crate::node::error::NodeError;
use node_actor::NodeActor;

/// Cluster Node
/// Starts server, actor system and creates nodes registry
pub struct ClusterNode {
    srv: Server,
    main_arbiter: ArbiterHandle,
    registry: Arc<RegistryCollection>,
    nodes: Arc<NodesRegistry>,
}

impl ClusterNode {
    /// Creates new node with provided config
    #[tracing::instrument]
    pub fn new(config: &NodeConfig) -> Result<ClusterNode, NodeError> {
        if !actix_rt::System::is_registered() {
            return Err(NodeError::ThreadDoesntHaveSystem)
        }
        
        let server_config = config.server_config();

        let nodes = Arc::new(NodesRegistry::new(
            config.this_node.node_id,
            &config.this_node.addr()));
        
        let registry = Arc::new(RegistryCollection::new(nodes.clone()));
        
        let srv = create_server(&server_config, registry.clone())?;
        
        Ok(ClusterNode{
            main_arbiter: Arbiter::current(),
            srv,
            registry,
            nodes,
        })
    }
    
    /// Runs node
    #[tracing::instrument(skip(self))]
    pub async fn run(&self) -> Result<(), NodeError> {
        let this_node = self.nodes.this_node();
        let this_id = this_node.id;
        self.nodes
            .register_actor::<NodeActor>(this_id, this_node.id)
            .await;
        
        let node_actor = self.registry.node_actors().get_or_activate_node(this_id).await?;
        let _ = node_actor.send(node_actor::Ping{}).await?; // activate node with ping
        
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
        let arb = self.main_arbiter.clone();
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
