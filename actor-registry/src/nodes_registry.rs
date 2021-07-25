use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

use std::sync::Arc;
use tokio::sync::RwLock;

use remote_actor::{RemoteActor, RemoteActorAddr};

use cluster_config::{NodeId, ClusterNodesConfigHandleFactory, ClusterNodesConfig, ClusterNodesConfigHandle};

pub enum ActorFromNodes<A: RemoteActor> {
    Remote(RemoteActorAddr<A>),
    Local,
    NotFound,
}

pub struct NodesRegistryFactory {
    config_factory: ClusterNodesConfigHandleFactory,
    a: Arc<RwLock<HashMap<String, NodeId>>>,
}

impl NodesRegistryFactory {
    pub fn new(config: &ClusterNodesConfig) -> NodesRegistryFactory {
        let a = HashMap::new();
        NodesRegistryFactory{
            config_factory: config.factory(),
            a: Arc::new(RwLock::new(a)),
        }
    }
    
    pub fn create(&self) -> NodesRegistry {
        NodesRegistry {
            config: self.config_factory.create(),
            a: self.a.clone(),
        }
    }
}

pub struct NodesRegistry {
    config: ClusterNodesConfigHandle,
    a: Arc<RwLock<HashMap<String, NodeId>>>,
}

impl NodesRegistry {
    pub fn this_node_id(&self) -> NodeId {
        self.config.this_node_id
    }
    
    pub async fn register_actor<A: RemoteActor>(&self, id: A::Id, node_id: NodeId) {
        let c = self.a.clone();
        let mut rw = c.write().await;
        let _ = rw.insert(Self::build_actor_id::<A>(id), node_id);
    }

    pub async fn get_actor<A: RemoteActor>(&self, id: A::Id) -> ActorFromNodes<A> {
        let a = self.a.clone();
        let rwa = a.read().await;

        let actor_id = Self::build_actor_id::<A>(id.clone());
        let node_id = rwa.get(&actor_id).copied(); // copy to release lock
        drop(rwa);

        match node_id {
            Some(node_id) => {
                if node_id == self.config.this_node_id {
                    return ActorFromNodes::Local;
                }
                
                match self.config.node_by_id(&node_id) {
                    Some(node) => ActorFromNodes::Remote(node.to_remote_actor_addr(id)),
                    None => ActorFromNodes::NotFound,
                }
            }
            None => ActorFromNodes::NotFound,
        }
    }

    pub fn get_members(&self) -> HashSet<NodeId> {
        self.config.all_node_ids()
    }
    
    fn build_actor_id<A: RemoteActor>(id: A::Id) -> String {
        format!("{}:{}", A::name(), id)
    }
}
