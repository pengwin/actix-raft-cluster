use thiserror::Error;
use cluster_config::NodeId;

/// Attach Node Error
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Node config not found
    #[error("Config for node {id} is not found")]
    NodeConfigIsNotFound {
        /// Leader Node Id
        id: NodeId,
    },
}
