use actix::{ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use remote_actor::RemoteMessage;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::info;

use super::NodeActor;
use super::{NodeActorError, NodeActorId};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub nodes: HashSet<NodeActorId>,
}

#[derive(Debug, Message, Serialize, Deserialize, RemoteMessage)]
#[rtype(result = "Result<NodeMetrics, NodeActorError>")]
pub struct Metrics {}

impl Handler<Metrics> for NodeActor {
    type Result = ResponseActFuture<Self, Result<NodeMetrics, NodeActorError>>;

    #[tracing::instrument(skip(self, _ctx))]
    fn handle(&mut self, _msg: Metrics, _ctx: &mut Context<Self>) -> Self::Result {
        let members = self.registry.get_members();
        Box::pin(
            async move { members }
                .into_actor(self)
                .map(move |res, _act, _ctx| {
                    info!("End operation");
                    Ok(NodeMetrics { nodes: res })
                }),
        )
    }
}
