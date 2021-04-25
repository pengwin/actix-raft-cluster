use actix::{ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use std::collections::HashSet;
use tracing::{info};
use remote_actor::RemoteMessage;
use serde::{Serialize, Deserialize};

use super::NodeActor;
use super::{NodeActorError, NodeActorId};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub nodes: HashSet<NodeActorId>
}

#[derive(Debug, Message, Serialize, Deserialize, RemoteMessage)]
#[rtype(result = "Result<NodeMetrics, NodeActorError>")]
pub struct Metrics {}


impl Handler<Metrics> for NodeActor {
    type Result = ResponseActFuture<Self, Result<NodeMetrics, NodeActorError>>;

    #[tracing::instrument(skip(self, _ctx))]
    fn handle(&mut self, _msg: Metrics, _ctx: &mut Context<Self>) -> Self::Result {
        let block_reg = self.registry.clone();
        Box::pin(async move {
            block_reg
                .get_members()
                .await
        }.into_actor(self).map(
            move |res, _act, _ctx| {
                info!("End operation");
                Ok(NodeMetrics{nodes:res})
            }
        ))
    }
}
