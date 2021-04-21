use node_actor::NodeActorId;

#[derive(Debug)]
pub struct RemoteNodeConfig {
    pub node_id: NodeActorId,
    pub protocol: &'static str,
    pub host: String,
    pub port: u16,
}

impl RemoteNodeConfig {
    pub fn addr(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}