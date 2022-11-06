use std::marker::PhantomData;

use super::{StatePersistenceError, StateSerializer, VirtualActorWithState};

#[derive(Default)]
pub struct JsonStateSerializer<V: VirtualActorWithState> {
    phantom_data: PhantomData<V>,
}

impl<V: VirtualActorWithState> StateSerializer<V> for JsonStateSerializer<V> {
    fn new() -> Self {
        JsonStateSerializer {
            phantom_data: PhantomData::default(),
        }
    }

    fn serialize_state(&self, state: &V::State) -> Result<Vec<u8>, StatePersistenceError> {
        serde_json::to_vec(state).map_err(|e| {
            let err = anyhow::Error::new(e).context("Serialization error");
            StatePersistenceError::SerializationError(err)
        })
    }

    fn deserialize_state(&self, data: &[u8]) -> Result<V::State, StatePersistenceError> {
        serde_json::from_slice(data).map_err(|e| {
            let err = anyhow::Error::new(e).context("Deserialization error");
            StatePersistenceError::Deserialization(err)
        })
    }
}
