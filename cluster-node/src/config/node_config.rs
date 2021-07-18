use std::collections::HashMap;

use node_actor::NodeActorId;
use actor_registry::{NodeItem, NodeId};

use crate::config::remote_node::{RemoteNodeConfig};
use crate::web_server::ServerConfig;

use super::ConfigError;

/// Node configuration
#[derive(Debug)]
pub struct NodeConfig {
    /// Cluster name
    pub cluster_name: String,
    /// Numbers of actix-web workers
    pub sever_workers_number: usize,
    /// Id of current node
    pub this_node_id: NodeActorId,
    /// Nodes configurations
    pub nodes: Vec<RemoteNodeConfig>,
}

pub(crate) struct NodesConfig {
    pub(crate) this_node: NodeItem,
    pub(crate) nodes: HashMap<NodeId, NodeItem>
}

impl NodeConfig {
    /// Creates new node configuration
    pub fn new(
        cluster_name: &str,
        this_node_id: NodeActorId,
        nodes: Vec<RemoteNodeConfig>,
    ) -> NodeConfig {
        NodeConfig {
            cluster_name: cluster_name.to_owned(),
            sever_workers_number: 4,
            this_node_id,
            nodes,
        }
    }

    pub(crate) fn server_config(&self) -> Result<ServerConfig, ConfigError> {
        self.get_this_node(|this_node| {
            let name = format!("{} {}", &self.cluster_name, self.this_node_id);
            ServerConfig::new(&name, self.sever_workers_number, this_node)
        })
    }

    pub(crate) fn nodes_config(&self) -> Result<NodesConfig, ConfigError> {
        self.get_this_node(|this_node| {
            let this_node = NodeItem::new(this_node.node_id, this_node.addr().as_str());
            let mut nodes = HashMap::new();
            for node in &self.nodes {
                nodes.insert(node.node_id, NodeItem::new(node.node_id, node.addr().as_str()));
            }
            NodesConfig{this_node, nodes}
        })
    }
    
    fn get_this_node<T>(&self, mapper: impl FnOnce(&RemoteNodeConfig) -> T) -> Result<T, ConfigError> {
        let node = self.nodes.iter().find(|n| n.node_id == self.this_node_id);
        match node {
            Some(this_node) => Ok(mapper(this_node)),
            None => Err(ConfigError::NodeConfigIsNotFound { id: self.this_node_id }) 
        }
    }
}
