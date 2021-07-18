use reqwest::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;
use tokio::sync::oneshot::error::RecvError;

#[derive(Error, Debug)]
pub struct RemoteActorHttpError {
    pub status: StatusCode,
    pub text: Option<String>,
    pub text_parsing_err: Option<reqwest::Error>,
}

impl Display for RemoteActorHttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

#[derive(Error, Debug)]
pub enum RemoteActorError {
    #[error("remote actor returned http Error({0:?}): '{0}'")]
    Http(
        #[from]
        #[source]
        RemoteActorHttpError,
    ),
    #[error("remote actor send Error({0:?}): '{0}'")]
    Send(
        #[from]
        #[source]
        reqwest::Error,
    ),
    #[error("remote actor response Error({0:?}): '{0}'")]
    Parsing(#[source] reqwest::Error),
}

#[derive(Error, Debug)]
pub enum ActorActivatorError {
    #[error("Actor creation Error({0:?}): '{0}'")]
    ActorCreationError(
        #[from]
        #[source]
        anyhow::Error,
    ),
    #[error("Receive Error({0:?}): '{0}'")]
    ReceiveError(
        #[from]
        #[source]
        RecvError,
    ),
}
