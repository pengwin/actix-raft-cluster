use std::hash::Hash;
use thiserror::Error;
use actix::MailboxError;
use std::fmt::{Display, Formatter, Result as FmtResult};

use remote_actor::{RemoteActorError, ActorActivatorError};

#[derive(Error, Debug)]
pub struct NodeNotFoundError {
    node_id: String,
}

impl Display for NodeNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Node '{}' not found", self.node_id)
    }
}

impl NodeNotFoundError {
    pub fn new<I>(node_id: I) -> NodeNotFoundError where
    I: 'static + Display + Clone + Eq + Hash {
        NodeNotFoundError { node_id: format!("{}", node_id) }
    }
}

#[derive(Error, Debug)]
pub enum ActorRegistryError {
    #[error("Activator Error({0:?}): '{0}'")]
    ActivatorError(#[from] ActorActivatorError),
    #[error("Local actor send Error({0:?}): '{0}'")]
    LocalSendError(#[from] MailboxError),
    #[error("Remote actor send Error({0:?}): '{0}'")]
    RemoteSendError(#[from] RemoteActorError),
    #[error("Actor not found Error")]
    NodeNotFound
}