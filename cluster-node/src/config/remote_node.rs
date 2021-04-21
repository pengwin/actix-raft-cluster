use node_actor::NodeActorId;

/// Config of remote node
#[derive(Debug)]
pub struct RemoteNodeConfig {
    /// Remote node Id 
    pub node_id: NodeActorId,
    /// Server network protocol (Http/Https)
    pub protocol: &'static str,
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