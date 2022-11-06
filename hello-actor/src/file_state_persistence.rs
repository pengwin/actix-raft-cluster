
use std::{sync::Arc, marker::PhantomData, path::Path};

use actix::Addr;
use async_trait::async_trait;
use tokio::{fs::{self}, io::AsyncWriteExt};
use anyhow::Context;

use crate::persistence::{StatePersistence, StatePersistenceError, VirtualActorWithState};

pub struct FileStatePersistenceFactory<V: VirtualActorWithState> {
    persistence: Arc<FileStatePersistence<V>>,
}

impl<V: VirtualActorWithState> FileStatePersistenceFactory<V> {
    pub fn new() -> Self {
        FileStatePersistenceFactory {
            persistence: Arc::new(FileStatePersistence {
                phantom_data: PhantomData::default()
            }),
        }
    }

    pub fn create(&self) -> Arc<FileStatePersistence<V>> {
        self.persistence.clone()
    }
}

pub struct FileStatePersistence<V: VirtualActorWithState> {
    phantom_data: PhantomData<Addr<V>>
}

impl<V: VirtualActorWithState> FileStatePersistence<V> {
    async fn prepare_dir(id: V::Id) -> Result<String, anyhow::Error> {
        let path = "./data/persistence/";
        fs::create_dir_all(path).await.with_context(|| format!("Creating dir {}", path))?;

        let path = Path::new(path).join(format!("{}_{}.json", V::name(), id));
        let path = path.to_str()
        .ok_or(anyhow::Error::msg("Empty path"))?;

        Ok(path.to_owned())
    }
}

#[async_trait]
impl<V: VirtualActorWithState> StatePersistence<V> for FileStatePersistence<V> {
    async fn save_state(&self, id: V::Id, state: Arc<[u8]>) -> Result<(), StatePersistenceError> {
        let id_clone = id.clone();
        let path = Self::prepare_dir(id_clone).await.map_err(|e| StatePersistenceError::SaveError(e))?;
        
        let mut f = fs::File::create(path).await.map_err(|e| {
            let err = anyhow::Error::new(e).context("Create file");
            StatePersistenceError::SaveError(err)
        })?;

        f.write_all(&state).await.map_err(|e| {
            let err = anyhow::Error::new(e).context("Write file");
            StatePersistenceError::SaveError(err)
        })?;

        Ok(())
    }

    async fn load_state(&self, id: V::Id) -> Result<Option<Arc<[u8]>>, StatePersistenceError> {
        let id_clone = id.clone();
        let path = Self::prepare_dir(id_clone).await.map_err(|e| StatePersistenceError::LoadError(e))?;

        if !Path::new(&path).exists() {
            return Ok(None)
        }
        
        let r = fs::read(&path).await.map_err(|e| {
            let err = anyhow::Error::new(e).context(format!("Reading file {}", path));
            StatePersistenceError::LoadError(err)
        })?;

        Ok(Some(r.into()))
    }
}
