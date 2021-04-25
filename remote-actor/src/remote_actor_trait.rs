use actix::{Actor, Context};

use crate::ActorId;

pub trait RemoteActorFactory<A: RemoteActor>: Send + Sync + 'static {
    fn create(&self, id: A::Id, ctx: &mut Context<A>) -> A;
}

pub trait RemoteActor: Actor + Actor<Context = Context<Self>> {
    type Id: ActorId;
    type Factory: RemoteActorFactory<Self>;
    
    fn name() -> &'static str;
    
    fn id(&self) -> Self::Id;
}