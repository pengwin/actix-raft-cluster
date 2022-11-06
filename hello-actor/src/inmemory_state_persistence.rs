use std::{collections::HashMap, sync::Arc};

use parking_lot::Mutex;

use async_trait::async_trait;

use crate::persistence::{StatePersistence, StatePersistenceError, VirtualActorWithState};

pub struct InmemoryStatePersistenceFactory<V: VirtualActorWithState> {
    persistence: Arc<InmemoryStatePersistence<V>>,
}

impl<V: VirtualActorWithState> InmemoryStatePersistenceFactory<V> {
    pub fn new() -> Self {
        InmemoryStatePersistenceFactory {
            persistence: Arc::new(InmemoryStatePersistence {
                states: Mutex::new(HashMap::new()),
            }),
        }
    }

    pub fn create(&self) -> Arc<InmemoryStatePersistence<V>> {
        self.persistence.clone()
    }
}

pub struct InmemoryStatePersistence<V: VirtualActorWithState> {
    states: Mutex<HashMap<V::Id, Arc<[u8]>>>,
}

#[async_trait]
impl<V: VirtualActorWithState> StatePersistence<V> for InmemoryStatePersistence<V> {
    async fn save_state(&self, id: V::Id, state: Arc<[u8]>) -> Result<(), StatePersistenceError> {
        let mut map = self.states.lock();
        map.insert(id, state);

        Ok(())
    }

    async fn load_state(&self, id: V::Id) -> Result<Option<Arc<[u8]>>, StatePersistenceError> {
        let map = self.states.lock();
        match map.get(&id) {
            Some(s) => Ok(Some(s.clone())),
            None => Ok(None),
        }
    }
}
