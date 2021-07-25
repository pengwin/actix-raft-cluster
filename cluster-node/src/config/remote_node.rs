use node_actor::NodeActorId;
use std::fmt::Formatter;

/// Known protocols for cluster node
#[derive(Debug, Clone)]
pub enum RemoteNodeConfigProtocol {
    /// Http protocol
    Http
}

impl std::fmt::Display for RemoteNodeConfigProtocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoteNodeConfigProtocol::Http => write!(f, "http"),
        }
    }
}

/// Config of remote node
#[derive(Debug, Clone)]
pub struct RemoteNodeConfig {
    /// Remote node Id
    pub node_id: NodeActorId,
    /// Server network protocol (Http/Https)
    pub protocol: RemoteNodeConfigProtocol,
    /// Server host
    pub host: String,
    ///Server port
    pub port: u16,
}

impl RemoteNodeConfig {
    /// Returns network address of node
    pub fn addr(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}
