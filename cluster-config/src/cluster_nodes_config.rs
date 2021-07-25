use std::collections::HashSet;

use crate::{NodeId, NodeItem};
use evmap::{ReadHandle, WriteHandle, ReadHandleFactory};
use std::sync::Arc;
use tokio::sync::Mutex;


/// Cluster config.
/// Because it's not async, it should use inmemory cache internally 
/// and lock free data structures
pub struct ClusterNodesConfig {
    pub this_node_id: NodeId,
    reader_factory: Arc<ReadHandleFactory<NodeId, NodeItem>>,
    writer: Arc<Mutex<WriteHandle<NodeId, NodeItem>>>
}

impl ClusterNodesConfig {
    pub fn new(this_node_id: NodeId, nodes: &Vec<NodeItem>) -> ClusterNodesConfig {
        let (r, mut w) = evmap::new();

        tracing::info!("Create cluster nodes config with  {} nodes", nodes.len());

        for node in nodes {
            w.insert(node.id, node.clone());
        }

        w.refresh();
        
        let reader_factory = Arc::new(r.factory());
        let writer = Arc::new(Mutex::new(w));

        ClusterNodesConfig { this_node_id, reader_factory, writer }
    }

    pub fn factory(&self) -> ClusterNodesConfigHandleFactory {
        ClusterNodesConfigHandleFactory {
            this_node_id: self.this_node_id,
            reader_factory: self.reader_factory.clone(),
        }
    }
    
    pub async fn add_node(&mut self, node: NodeItem) {
        let mut w = self.writer.lock().await;
        w.empty(node.id); // remove all values, to ensure single value
        w.insert(node.id, node.clone());
        w.refresh();
    }
}

pub struct ClusterNodesConfigHandleFactory {
    pub this_node_id: NodeId,
    reader_factory: Arc<ReadHandleFactory<NodeId, NodeItem>>,
}

impl ClusterNodesConfigHandleFactory {
    pub fn create(&self) -> ClusterNodesConfigHandle {
        let reader = self.reader_factory.handle();

        ClusterNodesConfigHandle{
            this_node_id: self.this_node_id.clone(),
            reader,
        }
    }
}

pub struct ClusterNodesConfigHandle {
    pub this_node_id: NodeId,
    reader: ReadHandle<NodeId, NodeItem>,
}

impl ClusterNodesConfigHandle {

    pub fn node_by_id(&self, id: &NodeId) -> Option<NodeItem> {
        let value =  self.reader.get_one(id)?;
        Some(value.clone())
    }

    pub fn all_node_ids(&self) -> HashSet<NodeId> {
        self.reader.map_into(|k, _| *k)
    }
}
