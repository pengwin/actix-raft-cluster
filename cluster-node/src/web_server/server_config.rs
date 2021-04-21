use std::time::Duration;
use crate::RemoteNodeConfig;

#[derive(Debug)]
pub struct ServerConfig {
    pub name: String,
    pub bind_point: String,
    pub(super) sever_workers_number: usize,
    pub(super) shutdown_timeout: Duration
}

impl ServerConfig {
    pub fn new(name: &str, sever_workers_number: usize, this_node: &RemoteNodeConfig) -> ServerConfig {
        ServerConfig {
            name: name.to_owned(),
            sever_workers_number,
            bind_point: format!("{}:{}", this_node.host, this_node.port),
            shutdown_timeout: Duration::from_secs(60),
        }
    }
}