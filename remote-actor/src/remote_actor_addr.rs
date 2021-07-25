use actix::dev::ToEnvelope;
use actix::Handler;
use reqwest::StatusCode;

use super::actor_path_builder::actor_url;
use super::error::{RemoteActorError, RemoteActorHttpError};
use super::RemoteActor;
use super::RemoteMessage;
use crate::{RemoteMessageResponse, ActorAddr};

pub struct RemoteActorAddr<A: RemoteActor> {
    id: A::Id,
    addr: ActorAddr,
}

impl<A: RemoteActor> Clone for RemoteActorAddr<A> {
    fn clone(&self) -> RemoteActorAddr<A> {
        RemoteActorAddr {
            id: self.id.clone(),
            addr: self.addr.clone(),
        }
    }
}

impl<A: RemoteActor> RemoteActorAddr<A> {
    pub fn new(id: A::Id, addr: ActorAddr) -> RemoteActorAddr<A> {
        RemoteActorAddr { id, addr }
    }

    pub fn id(&self) -> A::Id {
        self.id.clone()
    }

    #[tracing::instrument(skip(self, msg))]
    pub async fn send<M>(&self, msg: &M) -> Result<M::Result, RemoteActorError>
    where
        A: Handler<M>,
        M: RemoteMessage,
        M::Result: RemoteMessageResponse,
        A::Context: ToEnvelope<A, M>,
    {
        self.internal_send::<M>(&msg).await
    }
    #[tracing::instrument(skip(self, msg))]
    async fn internal_send<M>(&self, msg: &M) -> Result<M::Result, RemoteActorError>
    where
        A: Handler<M>,
        M: RemoteMessage,
        M::Result: RemoteMessageResponse,
        A::Context: ToEnvelope<A, M>,
    {
        /*let str_message = serde_json::to_string_pretty(&msg)?;
        log::info!("Sending {}", str_message);*/

        let client = reqwest::Client::new();
        let url = actor_url::<A, M>(self.addr.clone(), self.id());
        tracing::info!("Url: {}", url);
        let response = client
            .post(url)
            .json(msg)
            .send()
            .await
            .map_err(RemoteActorError::from)?;

        let status = response.status();

        match status {
            StatusCode::OK => {
                let r = response
                    .json::<M::Result>()
                    .await
                    .map_err(RemoteActorError::Parsing)?;
                Ok(r)
            }
            _ => {
                let txt = response.text().await;
                match txt {
                    Ok(text) => {
                        tracing::error!("Error sending message {} {}", status, text);
                        Err(RemoteActorError::Http(RemoteActorHttpError {
                            status,
                            text: Some(text),
                            text_parsing_err: None,
                        }))
                    }
                    Err(e) => {
                        tracing::error!("Error reading err message text {} {}", status, e);
                        Err(RemoteActorError::Http(RemoteActorHttpError {
                            status,
                            text: None,
                            text_parsing_err: Some(e),
                        }))
                    }
                }
            }
        }
    }
}
