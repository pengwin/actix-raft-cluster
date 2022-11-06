use async_trait::async_trait;
use thiserror::Error;

use actix::prelude::*;

use crate::persistence::StatePersistenceError;

#[derive(Error, Debug)]
pub enum StopRequestError {
    #[error("State Save {0}")]
    StateSaveError(#[from]StatePersistenceError)
}

#[derive(Message)]
#[rtype(result = "Result<(), StopRequestError>")]
pub struct StopRequest;

#[async_trait]
pub trait StoppableVirtualActor: Actor<Context = Context<Self>> + Handler<StopRequest> {
    //async fn stop() -> Result<(), StopRequestError>;
}