use std::sync::Arc;

use actix::{ResponseActFuture, fut, WrapFuture};

use super::{VirtualActorWithState, StatePersistence, StatePersistenceError, StateSerializer, JsonStateSerializer};

pub struct PersistentState<V, S = JsonStateSerializer<V>>
where V: VirtualActorWithState, S: StateSerializer<V> {
    id: V::Id,
    state: V::State,
    persistence: Arc<V::StatePersistence>,
    serializer: S
}

impl<V: VirtualActorWithState, S: StateSerializer<V>> PersistentState<V, S> {

    pub fn new(id: V::Id, persistence: Arc<V::StatePersistence>) -> PersistentState<V, S> {
        PersistentState { id, state: V::State::default(), persistence, serializer: S::new() }
    }

    pub fn id(&self) -> V::Id {
        self.id.clone()
    }

    pub async fn save(&self) -> Result<(), StatePersistenceError> {
        let vec = self.serializer.serialize_state(&self.state)?;
        self.persistence.save_state(self.id.clone(), vec.into()).await?;
        tracing::debug!("State saved for {} with id {}", V::name(), self.id);
        Ok(())
    }

    pub fn save_actor(&self, act: &V) -> ResponseActFuture<V, Result<(), StatePersistenceError>>  {
        match self.serializer.serialize_state(&self.state) {
            Err(e) => Box::pin(fut::err(e)),
            Ok(vec) => {
                let id = self.id.clone();
                let state = vec.into();
                let p = self.persistence.clone();
                Box::pin(async move { 
                    p.save_state(id.clone(), state).await?;
                    tracing::debug!("State saved for {} with id {}", V::name(), id);
                    Ok(())
                }.into_actor(act))
            }
        }
    }

    pub async fn load(&mut self) -> Result<(), StatePersistenceError> {
        let vec = self.persistence.load_state(self.id.clone()).await?;
        tracing::debug!("State loaded for {} with id {}", V::name(), self.id);
        let state = match vec {
            Some(s) => self.serializer.deserialize_state(&s)?,
            None => V::State::default(),
        };
        self.state = state;
        Ok(())
    }

    #[inline(always)]
    pub fn modify(&mut self, f: fn(&mut V::State)) {
        f(&mut self.state)
    }

    #[inline(always)]
    pub fn select<T>(&mut self, f: fn(&V::State) -> T) -> T {
        f(&self.state)
    }
}