use thiserror::Error;
use actor_registry::ActorRegistryError;
use remote_actor::RemoteActorError;
use node_actor::NodeActorError;
use crate::web_server::ServerError;

#[derive(Error, Debug)]
pub enum AttachError {
    #[error("Send Error Error({0:?}): '{0}'")]
    SendError(#[source] #[from]RemoteActorError),
    #[error("Attach Node Error({0:?}): '{0}'")]
    AttachNodeError(#[source] #[from] NodeActorError)
}

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Attach Error({0:?}): '{0}'")]
    AttachError(#[source] #[from]AttachError),
    #[error("ActorRegistryError Error({0:?}): '{0}'")]
    ActorRegistryError(#[source] #[from] ActorRegistryError),
    #[error("Server Start Error({0:?}): '{0}'")]
    ServerStartError(#[source] #[from] ServerError),
    #[error("Server Run Error({0:?}): '{0}'")]
    ServerRunError(#[source] #[from] std::io::Error),
    #[error("Thread Doesnt Have System")]
    ThreadDoesntHaveSystem
}