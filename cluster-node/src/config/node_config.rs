use node_actor::NodeActorId;
use cluster_config::{NodeItem, NodeId};

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
    pub(crate) this_node_id: NodeId,
    pub(crate) nodes: Vec<NodeItem>
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
    
    /// Returns copy of node with id == this_node_id
    pub fn this_node(&self) -> Option<RemoteNodeConfig> {
        match self.get_this_node(|e| e.clone()) {
            Ok(e) => Some(e),
            Err(_) => None,
        }
    }

    pub(crate) fn server_config(&self) -> Result<ServerConfig, ConfigError> {
        self.get_this_node(|this_node| {
            let name = format!("{} {}", &self.cluster_name, self.this_node_id);
            ServerConfig::new(&name, self.sever_workers_number, this_node)
        })
    }

    pub(crate) fn nodes_config(&self) -> NodesConfig {
        NodesConfig{
            this_node_id: self.this_node_id,
            nodes: self.nodes.iter()
                .map(|n| NodeItem::new(n.node_id, n.addr().as_str()))
                .collect()
        }
    }
    
    fn get_this_node<T>(&self, mapper: impl FnOnce(&RemoteNodeConfig) -> T) -> Result<T, ConfigError> {
        let node = self.nodes.iter().find(|n| n.node_id == self.this_node_id);
        match node {
            Some(this_node) => Ok(mapper(this_node)),
            None => Err(ConfigError::NodeConfigIsNotFound { id: self.this_node_id }) 
        }
    }
}
