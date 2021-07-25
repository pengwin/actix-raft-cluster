use actix::{ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use tracing::{error, info};

use super::NodeActor;
use crate::NodeActorInitializeError;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Init;

impl Handler<Init> for NodeActor {
    type Result = ResponseActFuture<Self, ()>;

    #[tracing::instrument(skip(self, _ctx))]
    fn handle(&mut self, _msg: Init, _ctx: &mut Context<Self>) -> Self::Result {
        info!("Start Init");
        let members = self.registry.get_members();
        info!("Cluster members {members}", members = members.len());
        Box::pin(
            async move {
                if members.is_empty() {
                    return Err(NodeActorInitializeError::new("Empty "));
                }
                Ok(())
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
