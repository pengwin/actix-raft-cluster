use crate::config::remote_node::RemoteNodeConfig;
use node_actor::NodeActorId;
use crate::web_server::ServerConfig;

#[derive(Debug)]
pub struct NodeConfig {
    pub cluster_name: String,
    pub sever_workers_number: usize,
    pub this_node: RemoteNodeConfig,
    pub leader_node: Option<RemoteNodeConfig>,
}

impl NodeConfig {
    pub fn new(
        cluster_name: &str,
        node_id: NodeActorId,
        host: &str,
        port: u16,
        leader_node_id: Option<NodeActorId>,
        leader_host: Option<String>,
        leader_port: Option<u16>,
    ) -> NodeConfig {
        let leader_node = match (leader_node_id, leader_host, leader_port) {
            (Some(n), Some(h), Some(p)) => Some(RemoteNodeConfig{
                node_id: n,
                host: h,
                port: p,
                protocol: "http",
            }),
            (_, _, _) => None
        };

        NodeConfig {
            cluster_name: cluster_name.to_owned(),
            sever_workers_number: 4,
            this_node: RemoteNodeConfig {
                node_id,
                host: host.to_owned(),
                port,
                protocol: "http"
            },
            leader_node,
        }
    }
    
    pub(crate) fn server_config(&self) -> ServerConfig {
        let name = format!("{} {}", &self.cluster_name, self.this_node.node_id);
        ServerConfig::new(&name, self.sever_workers_number, &self.this_node)
    }
}
