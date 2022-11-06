use std::sync::Arc;

use async_trait::async_trait;

use super::{VirtualActorWithState, StatePersistenceError};

#[async_trait]
pub trait StatePersistence<V: VirtualActorWithState>: std::marker::Send {
    async fn save_state(&self, id: V::Id, state: Arc<[u8]>) -> Result<(), StatePersistenceError>;
    async fn load_state(&self, id: V::Id) -> Result<Option<Arc<[u8]>>, StatePersistenceError>;
}