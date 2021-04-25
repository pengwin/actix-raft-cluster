use actix::{ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use std::collections::HashSet;
use tracing::{error, info};

use super::NodeActor;
use super::{NodeActorError, NodeActorErrorInner};

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Init;

impl Handler<Init> for NodeActor {
    type Result = ResponseActFuture<Self, ()>;

    #[tracing::instrument(skip(self, _ctx))]
    fn handle(&mut self, _msg: Init, _ctx: &mut Context<Self>) -> Self::Result {
        Box::pin(
            async move {
                let members = self.registry.get_members().await;
                
                for i in members.iter() {
                    let actor = self.registry.clone().get_actor()
                }
            }
            .into_actor(self)
            .map(move |res, _act, _ctx| match res {
                Ok(_) => {
                    info!("End operation");
                }
                Err(e) => {
                    error!("Init error {}", e);
                }
            }),
        )
    }
}
