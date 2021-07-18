use actix::{ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use remote_actor::RemoteMessage;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use super::NodeActor;
use super::NodeActorError;
use crate::NodeActorId;

#[derive(Serialize, Deserialize, Message, RemoteMessage)]
#[rtype(result = "Result<(), NodeActorError>")]
pub struct AttachNode {
    id: NodeActorId,
    addr: String,
}

impl AttachNode {
    pub fn new(id: NodeActorId, addr: String) -> AttachNode {
        AttachNode { id, addr }
    }
}

impl Handler<AttachNode> for NodeActor {
    type Result = ResponseActFuture<Self, Result<(), NodeActorError>>;

    #[tracing::instrument(name = "AttachNode", skip(self, msg, _ctx))]
    fn handle(&mut self, msg: AttachNode, _ctx: &mut Context<Self>) -> Self::Result {
        Box::pin(
            async move {
                info!("Adding node to registry {}", msg.id);
                //self.registry.add_node(msg.id, msg.addr.as_str()).await;
                Ok(())
            }
            .into_actor(self)
            .map(move |res, _act, _ctx| match res {
                Ok(_) => {
                    info!("Node attached");
                    Ok(())
                }
                Err(e) => {
                    error!("Node attach error: {}", e);
                    Err(e)
                }
            }),
        )
    }
}
