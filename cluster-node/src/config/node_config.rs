use crate::config::remote_node::RemoteNodeConfig;
use node_actor::NodeActorId;
use crate::web_server::ServerConfig;

/// Node configuration
#[derive(Debug)]
pub struct NodeConfig {
    /// Cluster name
    pub cluster_name: String,
    /// Numbers of actix-web workers
    pub sever_workers_number: usize,
    /// Configuration of current node
    pub this_node: RemoteNodeConfig,
}

impl NodeConfig {
    /// Creates new node configuration
    pub fn new(
        cluster_name: &str,
        node_id: NodeActorId,
        host: &str,
        port: u16,
    ) -> NodeConfig {
        

        NodeConfig {
            cluster_name: cluster_name.to_owned(),
            sever_workers_number: 4,
            this_node: RemoteNodeConfig {
                node_id,
                host: host.to_owned(),
                port,
                protocol: "http"
            }
        }
    }
    
    pub(crate) fn server_config(&self) -> ServerConfig {
        let name = format!("{} {}", &self.cluster_name, self.this_node.node_id);
        ServerConfig::new(&name, self.sever_workers_number, &self.this_node)
    }
}
