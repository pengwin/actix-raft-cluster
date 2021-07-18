use actor_registry::ActorRegistryError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub(super) struct NodeActorInitializeError {
    msg: String,
}

impl NodeActorInitializeError {
    pub fn new(msg: &str) -> NodeActorInitializeError {
        NodeActorInitializeError {
            msg: msg.to_owned(),
        }
    }
}

impl std::fmt::Display for NodeActorInitializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Error, Debug)]
pub(super) enum NodeActorErrorInner {
    #[error("Initialize Error({0:?}): '{0}'")]
    Initialize(
        #[source]
        #[from]
        NodeActorInitializeError,
    ),
    #[error("ActorRegistryError Error({0:?}): '{0}'")]
    ActorRegistryError(
        #[source]
        #[from]
        ActorRegistryError,
    ),
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub struct NodeActorError {
    msg: String,
}

impl std::fmt::Display for NodeActorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<NodeActorErrorInner> for NodeActorError {
    fn from(e: NodeActorErrorInner) -> Self {
        NodeActorError {
            msg: format!("{:?}", e),
        }
    }
}
