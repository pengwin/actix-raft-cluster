use thiserror::Error;

use crate::virtual_actor::VirtualActorFactoryError;

#[derive(Error, Debug)]
pub enum VirtualActorRegistryError {
    #[error("Factory is not set")]
    FactoryIsNotSet,
    #[error("Factory Error {0}")]
    FactoryError(#[from] VirtualActorFactoryError),
}
