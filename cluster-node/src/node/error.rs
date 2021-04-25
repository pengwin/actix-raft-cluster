use thiserror::Error;
use actor_registry::{ActorRegistryError, NodeId};
use remote_actor::RemoteActorError;
use node_actor::NodeActorError;
use crate::web_server::ServerError;
use tokio::sync::oneshot::error::RecvError;

/// Attach Node Error
#[derive(Error, Debug)]
pub enum AttachError {
    /// Network communication error
    #[error("Send Error Error({0:?}): '{0}'")]
    SendError(#[source] #[from]RemoteActorError),
    /// Attach logic Error
    #[error("Attach Node Error({0:?}): '{0}'")]
    AttachNodeError(#[source] #[from] NodeActorError),
    /// Channel Receive Error
    #[error("Receive Error({0:?}): '{0}'")]
    ReceiveError(#[source] #[from] RecvError),
    /// Leader node is not found
    #[error("Leader Node {id} is not found")]
    LeaderNodeIsNotFound{
        /// Leader Node Id
        id: NodeId
    },
}

/// Node General Error
#[derive(Error, Debug)]
pub enum NodeError {
    /// Attach to Leader Error
    #[error("Attach to Leader Error({0:?}): '{0}'")]
    AttachToLeaderError(#[source] #[from]AttachError),
    /// Error of Actors Registry
    #[error("Actor Registry Error Error({0:?}): '{0}'")]
    ActorRegistryError(#[source] #[from] ActorRegistryError),
    /// Error of server start
    #[error("Server Start Error({0:?}): '{0}'")]
    ServerStartError(#[source] #[from] ServerError),
    /// Error of server run
    #[error("Server Run Error({0:?}): '{0}'")]
    ServerRunError(#[source] #[from] std::io::Error),
    /// Error if actix system is not registered
    #[error("Thread Doesnt Have System")]
    ThreadDoesntHaveSystem
}