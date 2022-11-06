use thiserror::Error;
use actix::prelude::*;

#[derive(Error, Debug)]
pub enum SendErrorWrapper {
    #[error("Full")]
    Full,
    #[error("Closed")]
    Closed,
}

impl SendErrorWrapper {
    pub fn from_send_error<M>(e: SendError<M>) -> SendErrorWrapper {
        match e {
            SendError::Full(_) => SendErrorWrapper::Full,
            SendError::Closed(_) => SendErrorWrapper::Closed,
        }
    }
}


#[derive(Error, Debug)]
pub enum VirtualActorSendError {
    #[error("Send Error ({0:?}): '{0}'")]
    SendError(
        #[from]
        SendErrorWrapper,
    ),
    #[error("Mailbox Error ({0:?}): '{0}'")]
    MailboxError(
        #[from]
        MailboxError,
    ),
    #[error("Send Error MissingLocalActor")]
    MissingLocalActor,
}