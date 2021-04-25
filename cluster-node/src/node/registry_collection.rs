use node_actor::{NodeActorRegistry, NodeActorFactory};
use actor_registry::NodesRegistry;
use std::sync::Arc;

pub struct RegistryCollection {
    n: Arc<NodeActorRegistry>
}

impl RegistryCollection {
    pub fn new(nodes: Arc<NodesRegistry>) -> RegistryCollection {
        let node_actor_factory = NodeActorFactory::new(nodes.clone());
        let node_actor_registry = NodeActorRegistry::new(nodes, node_actor_factory);

        RegistryCollection{
            n: Arc::new(node_actor_registry),
        }
    }

    pub fn stop(&self) -> bool {
        self.n.stop()
    }
    
    pub fn node_actors(&self) -> Arc<NodeActorRegistry> {
        self.n.clone()
    }
}