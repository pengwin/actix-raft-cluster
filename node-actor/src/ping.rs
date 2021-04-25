use actix::{Context, Handler, Message};
use remote_actor::RemoteMessage;
use serde::{Serialize, Deserialize};

use super::NodeActor;
use crate::NodeActorError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pong;

#[derive(Debug, Message, Serialize, Deserialize, RemoteMessage)]
#[rtype(result = "Result<Pong, NodeActorError>")]
pub struct Ping {}

impl Handler<Ping> for NodeActor {
    type Result = Result<Pong, NodeActorError>;

    #[tracing::instrument(skip(self, _ctx))]
    fn handle(&mut self, _msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(Pong{})
    }
}
