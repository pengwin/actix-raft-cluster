use thiserror::Error;
use serde::{Serialize, Deserialize};
use actor_registry::ActorRegistryError;

#[derive(Error, Debug)]
pub(super) enum NodeActorErrorInner {
    #[error("Initialize Error({0:?}): '{0}'")]
    Initialize(#[source] #[from]anyhow::Error),
    #[error("ActorRegistryError Error({0:?}): '{0}'")]
    ActorRegistryError(#[source] #[from] ActorRegistryError)
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub struct NodeActorError {
    msg: String
}

impl std::fmt::Display for NodeActorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<NodeActorErrorInner> for NodeActorError {
    fn from(e: NodeActorErrorInner) -> Self {
        NodeActorError {
            msg: format!("{}", e)
        }
    }
}