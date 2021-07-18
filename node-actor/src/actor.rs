use actix::{Actor, AsyncContext, Context};
use remote_actor::{RemoteActor, RemoteActorFactory};

use crate::Init;
use actor_registry::{NodesRegistry, NodesRegistryFactory};
use std::sync::Arc;

type NodeActorId = u64;

pub struct NodeActor {
    pub(super) id: NodeActorId,
    pub(super) registry: NodesRegistry,
}

impl Actor for NodeActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.address().do_send(Init {})
    }
}

impl RemoteActor for NodeActor {
    type Id = NodeActorId;
    type Factory = NodeActorFactory;

    fn name() -> &'static str {
        stringify!(NodeActor)
    }

    fn id(&self) -> NodeActorId {
        self.id
    }
}

pub struct NodeActorFactory {
    registry_factory: Arc<NodesRegistryFactory>,
}

impl NodeActorFactory {
    pub fn new(registry_factory: Arc<NodesRegistryFactory>) -> NodeActorFactory {
        NodeActorFactory { registry_factory }
    }
}

impl RemoteActorFactory<NodeActor> for NodeActorFactory {
    fn create(
        &self,
        id: <NodeActor as RemoteActor>::Id,
        _ctx: &mut Context<NodeActor>,
    ) -> NodeActor {
        NodeActor {
            id,
            registry: self.registry_factory.create(),
        }
    }
}
