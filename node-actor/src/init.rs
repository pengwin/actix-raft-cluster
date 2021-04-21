use actix::{ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use std::collections::HashSet;
use tracing::{error, info};

use super::NodeActor;
use super::{NodeActorError, NodeActorErrorInner};
use crate::NodeActorId;

#[derive(Debug, Message)]
#[rtype(result = "Result<(), NodeActorError>")]
pub struct Init {
    members: HashSet<NodeActorId>,
}

impl Init {
    pub fn new(members: HashSet<NodeActorId>) -> Init {
        Init { members }
    }
}

impl Handler<Init> for NodeActor {
    type Result = ResponseActFuture<Self, Result<(), NodeActorError>>;

    #[tracing::instrument(skip(self, _ctx))]
    fn handle(&mut self, _msg: Init, _ctx: &mut Context<Self>) -> Self::Result {
        Box::pin(async move { anyhow::Result::Ok(()) }.into_actor(self).map(
            move |res, _act, _ctx| match res {
                Ok(_) => {
                    info!("End operation");
                    Ok(())
                }
                Err(e) => {
                    error!("Init error {}", e);
                    Err(NodeActorErrorInner::Initialize(e).into())
                }
            },
        ))
    }
}
