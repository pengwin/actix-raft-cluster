use structopt::StructOpt;

use cluster_node::{NodeConfig, RemoteNodeConfig};

#[derive(Debug, StructOpt)]
#[structopt(name = "cluster_node")]
pub struct ClusterConfig {
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub host: String,

    #[structopt(short, long, default_value = "8080")]
    pub port: u16,

    #[structopt(short, long)]
    pub node_id: u64,

    #[structopt(long)]
    pub leader_id: Option<u64>,

    #[structopt(long)]
    pub leader_host: Option<String>,

    #[structopt(long)]
    pub leader_port: Option<u16>,

    #[structopt(short, long, default_value = "primary-raft-group")]
    pub cluster_name: String,
}

impl ClusterConfig {
    pub fn read_from_args() -> ClusterConfig {
        ClusterConfig::from_args()
    }
    
    pub fn node_config(&self) -> NodeConfig {
        NodeConfig::new(
            &self.cluster_name,
            self.node_id,
            &self.host,
            self.port)
    }
    
    pub fn leader_config(&self) -> Option<RemoteNodeConfig> {
        match (self.leader_id, self.leader_host.clone(), self.leader_port) {
            (Some(n), Some(h), Some(p)) => Some(RemoteNodeConfig{
                node_id: n,
                host: h,
                port: p,
                protocol: "http",
            }),
            (_, _, _) => None
        }
    }
}

impl ToString for ClusterConfig {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
