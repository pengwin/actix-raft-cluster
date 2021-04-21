use actix::{Actor, Context};
use remote_actor::{RemoteActor, RemoteActorFactory};

use actor_registry::NodesRegistry;

type NodeActorId = u64;

pub struct NodeActor {
    pub(super) id: NodeActorId,
    pub(super) registry: NodesRegistry,
}

impl Actor for NodeActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {}
}

#[derive(Clone)]
pub struct NodeActorFactory {
    registry: NodesRegistry
}

impl NodeActorFactory {
    pub fn new(registry: NodesRegistry) -> NodeActorFactory {
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
