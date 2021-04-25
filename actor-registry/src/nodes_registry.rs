use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use remote_actor::{RemoteActor, RemoteActorAddr};

/// Node Id
pub type NodeId = u64;

pub struct NodeItem {
    pub id: NodeId,
    pub addr: String,
}

impl Display for NodeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(id {})", self.id)
    }
}

impl NodeItem {
    fn to_remote_actor_addr<A: RemoteActor>(&self, id: A::Id) -> RemoteActorAddr<A> {
        RemoteActorAddr::<A>::new(id, self.addr.clone())
    }
}

impl NodeItem {
    fn new(id: NodeId, addr: &str) -> NodeItem {
        NodeItem { id, addr: addr.to_owned() }
    }
}

pub enum ActorFromNodes<A: RemoteActor> {
    Remote(RemoteActorAddr<A>),
    Local,
    NotFound
}

pub struct NodesRegistry {
    current_node_id: NodeId,
    current_addr: String,
    n: Arc<RwLock<HashMap<NodeId, NodeItem>>>,
    a: Arc<RwLock<HashMap<String, NodeId>>>,
}

impl NodesRegistry {
    pub fn new(current_node_id: NodeId, current_addr: &str) -> NodesRegistry {
        let node = NodeItem::new(current_node_id, current_addr);
        let mut n = HashMap::new();
        n.insert(current_node_id, node);
        
        let a = HashMap::new();
        NodesRegistry {
            current_node_id,
            current_addr: current_addr.to_owned(),
            n: Arc::new(RwLock::new(n)),
            a: Arc::new(RwLock::new(a)),
        }
    }
    
    pub fn this_node(&self) -> NodeItem {
        NodeItem{
            id: self.current_node_id,
            addr: self.current_addr.clone(),
        }
    }
    
    pub async fn add_node(&self, node_id: NodeId, addr: &str) {
        let c = self.n.clone();
        let mut rw = c.write().await;
        let _ = rw.insert(node_id, NodeItem::new(node_id, addr));
    }

    pub async fn register_actor<A: RemoteActor>(&self, id: A::Id, node_id: NodeId) {
        let c = self.a.clone();
        let mut rw = c.write().await;
        let _ = rw.insert(Self::get_actor_id::<A>(id), node_id);
    }
    
    pub async fn get_actor<A: RemoteActor>(&self, id: A::Id) -> ActorFromNodes<A> {
        let a = self.a.clone();
        let rwa = a.read().await;
        
        let actor_id = Self::get_actor_id::<A>(id.clone());
        let node_id = rwa.get(&actor_id).copied(); // copy to release lock
        drop(rwa);
        
        match node_id {
            Some(node_id) => {
                if node_id == self.current_node_id {
                    return ActorFromNodes::Local
                }
                
                let n = self.n.clone();
                let rwn = n.read().await;
                match rwn.get(&node_id) {
                    Some(node) => ActorFromNodes::Remote(node.to_remote_actor_addr(id)),
                    None => ActorFromNodes::NotFound
                }
            },
            None => ActorFromNodes::NotFound,
        }
    }

    pub async fn get_members(&self) -> HashSet<NodeId> {
        let c = self.n.clone();
        let rw = c.read().await;
        let mut res = HashSet::new();
        for k in rw.keys() {
            res.insert(k.to_owned());
        }
        res
    }
    
    pub fn current_node(&self) -> NodeItem {
        NodeItem{
            id: self.current_node_id,
            addr: self.current_addr.clone(),
        }
    }
    
    fn get_actor_id<A: RemoteActor>(id: A::Id) -> String {
        format!("{}:{}", A::name(), id)
    }
}
