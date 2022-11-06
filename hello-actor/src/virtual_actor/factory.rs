use async_trait::async_trait;
use crate::persistence::StatePersistenceError;

use super::virtual_actor::VirtualActor;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum VirtualActorFactoryError {
    #[error("StateManagerError {0}")]
    StatePersistenceError(#[from]StatePersistenceError)
}

#[async_trait]
pub trait VirtualActorFactory<V: VirtualActor>:  Unpin + 'static {
    async fn create(&self, id: V::Id) -> Result<V, VirtualActorFactoryError>;
}