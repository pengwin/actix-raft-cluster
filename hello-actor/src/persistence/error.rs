use thiserror::Error;

#[derive(Error, Debug)]
pub enum StatePersistenceError {
    #[error("Save Error {0}")]
    SaveError(anyhow::Error),
    #[error("Load Error {0}")]
    LoadError(anyhow::Error),
    #[error("Serialization Error {0}")]
    SerializationError(anyhow::Error),
    #[error("Deserialization Error {0}")]
    Deserialization(anyhow::Error),
}