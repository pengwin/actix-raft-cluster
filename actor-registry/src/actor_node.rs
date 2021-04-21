use actix::{Addr, Actor, Handler};
use remote_actor::{RemoteActorAddr, RemoteActor, RemoteMessage, RemoteMessageResponse};
use actix::dev::ToEnvelope;

use super::ActorRegistryError;

pub enum ActorNode<A: RemoteActor> {
    Local(Addr<A>),
    Remote(RemoteActorAddr<A>)
}

impl<A: RemoteActor> Clone for ActorNode<A> {
    fn clone(&self) -> ActorNode<A> {
        match self {
            ActorNode::Local(l) => ActorNode::Local(l.clone()),
            ActorNode::Remote(r) => ActorNode::Remote(r.clone())
        }
    }
}

impl<A: RemoteActor> ActorNode<A> {
    pub async fn send<M>(&self, msg: M) -> Result<M::Result, ActorRegistryError> where
        A: Actor + Handler<M>,
        M: RemoteMessage,
        M::Result: RemoteMessageResponse,
        A::Context: ToEnvelope<A, M> {
            match self {
                ActorNode::Local(l) => {
                    tracing::info!("Start local send {}/{}", A::name(), M::name());
                    let ar = l.send(msg).await;
                    match ar {
                        Ok(r) => Ok(r),
                        Err(e) => Err(ActorRegistryError::from(e)),
                    }
                },
                ActorNode::Remote(r) => {
                    tracing::info!("Start remote send {}/{}", A::name(), M::name());
                    r.send(&msg).await.map_err(ActorRegistryError::from)
                }
            }
        }
}
