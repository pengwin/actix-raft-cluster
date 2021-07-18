use std::collections::{HashMap, HashSet};

use crate::{NodeId, NodeItem};
use evmap::{ReadHandle, WriteHandle, ReadHandleFactory};


/// Cluster config.
/// Because it's not async, it should use inmemory cache internally 
/// and lock free data structures
pub struct ClusterNodesConfig {
    pub this_node_id: NodeId,
    reader: ReadHandle<NodeId, NodeItem>,
    writer: WriteHandle<NodeId, NodeItem>
}

impl ClusterNodesConfig {
    pub fn new(this_node: NodeItem, nodes: HashMap<NodeId, NodeItem>) -> ClusterNodesConfig {
        let (reader, mut writer) = evmap::new();

        for (id, node) in nodes {
            writer.insert(id, node);
        }

        writer.refresh();

        ClusterNodesConfig { this_node_id: this_node.id, reader, writer }
    }

    pub fn factory(&self) -> ClusterNodesConfigHandleFactory {
        ClusterNodesConfigHandleFactory {
            this_node_id: self.this_node_id,
            reader_factory: self.reader.factory(),
        }
    }
}

pub struct ClusterNodesConfigHandleFactory {
    pub this_node_id: NodeId,
    reader_factory: ReadHandleFactory<NodeId, NodeItem>,
}

impl ClusterNodesConfigHandleFactory {
    pub fn create(&self) -> ClusterNodesConfigHandle {
        ClusterNodesConfigHandle{
            this_node_id: self.this_node_id.clone(),
            reader: self.reader_factory.handle()
        }
    }
}

pub struct ClusterNodesConfigHandle {
    pub this_node_id: NodeId,
    reader: ReadHandle<NodeId, NodeItem>,
}

impl ClusterNodesConfigHandle {

    pub fn node_by_id(&self, id: &NodeId) -> Option<NodeItem> {
        let values =  self.reader.get(id)?;
        values.iter().next().cloned()
    }

    pub fn all_node_ids(&self) -> HashSet<NodeId> {
        self.reader.map_into(|k, _| *k)
    }
}
