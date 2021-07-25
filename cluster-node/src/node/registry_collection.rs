use actor_registry::NodesRegistryFactory;
use node_actor::{NodeActorFactory, NodeActorRegistryFactory};
use std::sync::Arc;

pub struct RegistryCollection {
    n_factory: Arc<NodeActorRegistryFactory>,
}

impl RegistryCollection {
    pub fn new(nodes_registry_factory: Arc<NodesRegistryFactory>) -> RegistryCollection {
        let node_actor_factory = NodeActorFactory::new(nodes_registry_factory.clone());
        let node_actor_registry_factory = NodeActorRegistryFactory::new(nodes_registry_factory.clone(), node_actor_factory);

        RegistryCollection {
            n_factory: Arc::new(node_actor_registry_factory),
        }
    }

    pub fn stop(&self) -> bool {
        self.n_factory.stop()
    }

    pub fn node_actors_factory(&self) -> Arc<NodeActorRegistryFactory> {
        self.n_factory.clone()
    }
}
