use super::remote_actor_trait::RemoteActor;
use super::remote_message::{RemoteMessage, RemoteMessageResponse};
use actix::{Handler, Message};

pub fn actor_url_template<A, M>() -> String
where
    A: Handler<M> + RemoteActor,
    M: RemoteMessage,
    M::Result: RemoteMessageResponse,
{
    let id = "{id}";
    format!(
        "/api/actor/{actor}/{id}/{message}",
        actor = A::name(),
        id = id,
        message = M::name()
    )
}

pub fn actor_url<A, M>(url: String, id: A::Id) -> String
where
    A: Handler<M> + RemoteActor,
    M: Message + RemoteMessage,
    M::Result: RemoteMessageResponse,
{
    format!(
        "{url}/api/actor/{actor}/{id}/{message}",
        url = url,
        actor = A::name(),
        id = id,
        message = M::name()
    )
}
