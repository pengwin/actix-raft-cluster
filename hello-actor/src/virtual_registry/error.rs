use actix::MailboxError;
use actix_rt::task::JoinError;
use thiserror::Error;

use crate::virtual_actor::{VirtualActorFactoryError, StopRequestError};

#[derive(Error, Debug)]
pub enum VirtualActorRegistryError {
    #[error("Factory is not set")]
    FactoryIsNotSet,
    #[error("Factory Error {0}")]
    FactoryError(#[from] VirtualActorFactoryError),
    #[error("Stop All Actors Error {0:?}")]
    StopAllErrors(Vec<StopAllError>),
}

#[derive(Error, Debug)]
pub enum StopAllError {
    #[error("Join Error {0}")]
    JoinError(#[from] JoinError),
    #[error("Mailbox Error {0}")]
    MailboxError(#[from] MailboxError),
    #[error("StopRequest Error {0}")]
    StopRequestError(#[from] StopRequestError),
}
