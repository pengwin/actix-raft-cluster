use structopt::StructOpt;

use crate::config::ClusterConfig;
use cluster_node::{NodeConfig, RemoteNodeConfig, RemoteNodeConfigProtocol};

#[derive(Debug, StructOpt)]
#[structopt(name = "cluster_node")]
pub struct NodeArgConfig {
    #[structopt(short, long)]
    pub node_id: u64,

    #[structopt(short, long)]
    pub config_file: String,
}

impl NodeArgConfig {
    pub fn read_from_args() -> NodeArgConfig {
        NodeArgConfig::from_args()
    }

    pub fn node_config(&self, cfg: &ClusterConfig) -> Result<NodeConfig, std::io::Error> {
        let node = cfg.nodes.iter().find(|n| n.id == self.node_id);
        let nodes = cfg.nodes.iter().map(|n| RemoteNodeConfig{
            node_id: n.id,
            host: n.host.clone(),
            port: n.port,
            protocol: RemoteNodeConfigProtocol::Http
        }).collect();
        match node {
            Some(n) => Ok(NodeConfig::new(
                &cfg.cluster_name,
                n.id,
                nodes,
            )),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Node {} is not found", self.node_id),
            )),
        }
    }
}

impl ToString for NodeArgConfig {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
