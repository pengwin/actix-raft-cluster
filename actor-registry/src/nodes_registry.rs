use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

type NodeId = u64;

#[derive(Clone)]
pub(super) struct ClusterNode {
    id: NodeId,
    addr: String,
}

impl Display for ClusterNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(id {})", self.id)
    }
}

impl ClusterNode {
    fn new(id: NodeId, addr: &str) -> ClusterNode {
        ClusterNode { id, addr: addr.to_owned() }
    }
}

#[derive(Clone)]
pub struct NodesRegistry {
    n: Arc<RwLock<HashMap<NodeId, ClusterNode>>>,
    a: Arc<RwLock<HashMap<&'static str, NodeId>>>,
}

impl NodesRegistry {
    pub fn new(current_node_id: NodeId, current_addr: &str) -> NodesRegistry {
        let node = ClusterNode::new(current_node_id, current_addr);
        let mut n = HashMap::new();
        n.insert(current_node_id, node);
        
        let a = HashMap::new();
        NodesRegistry {
            n: Arc::new(RwLock::new(n)),
            a: Arc::new(RwLock::new(a)),
        }
    }
    
    pub async fn attach_node(&self, node_id: NodeId, addr: &str) {
        let c = self.n.clone();
        let mut rw = c.write().await;
        let _ = rw.insert(node_id, ClusterNode::new(node_id, addr));
    }

    pub async fn register_actor(&self, actor_id: &'static str, node_id: NodeId) {
        let c = self.a.clone();
        let mut rw = c.write().await;
        let _ = rw.insert(actor_id, node_id);
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
}
