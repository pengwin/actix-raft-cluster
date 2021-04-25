use actix::{Actor, Context, AsyncContext};
use remote_actor::{RemoteActor, RemoteActorFactory};

use actor_registry::NodesRegistry;
use std::sync::Arc;
use crate::Init;

type NodeActorId = u64;

pub struct NodeActor {
    pub(super) id: NodeActorId,
    pub(super) registry: Arc<NodesRegistry>,
}

impl Actor for NodeActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.address().do_send(Init{})
    }
}

pub struct NodeActorFactory {
    registry: Arc<NodesRegistry>
}

impl NodeActorFactory {
    pub fn new(registry: Arc<NodesRegistry>) -> NodeActorFactory {
        NodeActorFactory{registry}
    }
}

impl RemoteActorFactory<NodeActor> for NodeActorFactory {
    fn create(&self, id: <NodeActor as RemoteActor>::Id, _ctx: &mut Context<NodeActor>) -> NodeActor {
        NodeActor { id, registry: self.registry.clone() }
    }
}

impl RemoteActor for NodeActor {
    type Id = NodeActorId;
    type Factory = NodeActorFactory;
    
    fn name() -> &'static str {
        "NodeActor"
    }

    fn id(&self) -> NodeActorId {
        self.id
    }
}
