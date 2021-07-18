use actix::{Arbiter, ArbiterHandle};
use actix_web::dev::Server;
use std::sync::Arc;
use tracing::{info_span, Instrument};

use actor_registry::{NodesRegistryFactory, ClusterNodesConfig};

use crate::config::NodeConfig;
use crate::web_server::create_server;

use super::RegistryCollection;
use crate::node::error::NodeError;
use node_actor::NodeActor;

/// Cluster Node
/// Starts server, actor system and creates nodes registry
pub struct ClusterNode {
    srv: Server,
    main_arbiter: ArbiterHandle,
    registry: Arc<RegistryCollection>,
    nodes_registry_factory: Arc<NodesRegistryFactory>,
}

impl ClusterNode {
    /// Creates new node with provided config
    #[tracing::instrument]
    pub fn new(config: &NodeConfig) -> Result<ClusterNode, NodeError> {
        if !actix_rt::System::is_registered() {
            return Err(NodeError::ThreadDoesntHaveSystem);
        }

        let server_config = config.server_config()
            .map_err(NodeError::from)?;

        let nodes_config = config.nodes_config()
            .map_err(NodeError::from)?;
        
        let cluster_config = ClusterNodesConfig::new(nodes_config.this_node, nodes_config.nodes);

        let nodes_registry_factory = Arc::new(NodesRegistryFactory::new(&cluster_config));

        let registry = Arc::new(RegistryCollection::new(nodes_registry_factory.clone()));

        let srv = create_server(&server_config, registry.clone())?;

        Ok(ClusterNode {
            main_arbiter: Arbiter::current(),
            srv,
            registry,
            nodes_registry_factory,
        })
    }

    /// Runs node
    #[tracing::instrument(skip(self))]
    pub async fn run(&self) -> Result<(), NodeError> {
        let nodes_registry = self.nodes_registry_factory.create();

        let this_id = nodes_registry.this_node_id();
        
        nodes_registry.register_actor::<NodeActor>(this_id, this_id)
            .await;

        let node_actor = self
            .registry
            .node_actors_factory()
            .create()
            .get_or_activate_node(this_id)
            .await?;
        let _ = node_actor.send(node_actor::Ping {}).await?; // activate node with ping

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
