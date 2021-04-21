use node_actor::{NodeActorRegistry, NodeActorFactory};
use actor_registry::NodesRegistry;

#[derive(Clone)]
pub struct RegistryCollection {
    pub n: NodeActorRegistry,
}

impl RegistryCollection {
    pub fn new(nodes: NodesRegistry) -> RegistryCollection {
        let node_actor_factory = NodeActorFactory::new(nodes.clone());
        let node_actor_registry = NodeActorRegistry::new(nodes,
                                                         node_actor_factory);

        RegistryCollection{
            n: node_actor_registry,
        }
    }

    pub fn stop(&self) -> bool {
        self.n.stop()
    }
}