use super::{VirtualActorWithState, StatePersistenceError};

pub trait StateSerializer<V: VirtualActorWithState> {
    fn new() -> Self;
    fn serialize_state(&self, state: &V::State) -> Result<Vec<u8>, StatePersistenceError>;
    fn deserialize_state(&self, data: &[u8]) -> Result<V::State, StatePersistenceError>;
}