use actix::{dev::ToEnvelope, prelude::*};
use actix_web::{web, HttpResponse, Responder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

use crate::{
    virtual_actor::{VirtualActor, VirtualMessage, VirtualActorSendError},
    virtual_registry::{GetActor, VirtualActorRegistryActor, VirtualActorRegistryError},
};


#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("Virtual Actor SendError ({0:?}): '{0}'")]
    VirtualActorSendError(
        #[source]
        #[from]
        VirtualActorSendError,
    ),
    #[error("Virtual Actor RegistryError ({0:?}): '{0}'")]
    VirtualActorRegistryError(
        #[source]
        #[from]
        VirtualActorRegistryError,
    ),
    #[error("Mailbox Error ({0:?}): '{0}'")]
    MailboxError(
        #[source]
        #[from]
        MailboxError,
    ),
}

#[derive(Deserialize)]
struct Request<V: VirtualActor> {
    pub id: V::Id,
}

async fn send_message<V, M>(request: Request<V>, message: M) -> Result<M::Result, HandlerError>
where
    V: VirtualActor,
    M: VirtualMessage + Send + 'static,
    M::Result: Send + Serialize + DeserializeOwned,
    V: Handler<M>,
    V::Context: ToEnvelope<V, M>,
{
    let id = request.id.to_owned();
    let reg = VirtualActorRegistryActor::<V>::from_registry();
    let addr = reg.send(GetActor::new(id)).await??.addr;
    let res = addr.send(message).await?;
    Ok(res)
}

async fn handler<V, M>(path: web::Path<Request<V>>, body: web::Json<M>) -> impl Responder
where
    V: VirtualActor,
    M: VirtualMessage + Send + 'static,
    M::Result: Send + Serialize + DeserializeOwned,
    V: Handler<M>,
    V::Context: ToEnvelope<V, M>,
{
    let request = path.into_inner();
    let message = body.0;
    match send_message(request, message).await {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(err) => {
            let serde_err = serde_error::Error::new(&err);
            HttpResponse::InternalServerError().json(serde_err)
        },
    }
}

pub fn config<V, M>(cfg: &mut web::ServiceConfig)
where
    V: VirtualActor,
    M: VirtualMessage + Send + 'static,
    M::Result: Send + Serialize + DeserializeOwned,
    V: Handler<M>,
    V::Context: ToEnvelope<V, M>,
{
    let path = format!("/actor/{}/{{id}}/{}", V::name(), M::name());
    cfg.service(web::resource(path).route(web::post().to(handler::<V, M>)));
}
