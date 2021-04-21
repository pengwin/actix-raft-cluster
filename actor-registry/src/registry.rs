use actix::{Addr, ArbiterHandle, Arbiter};
use remote_actor::{RemoteActor, RemoteActorFactory};
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{ActorNode, ActorRegistryError};
use crate::{NodesRegistry};

pub struct ActorRegistry<A: RemoteActor> {
    name: String,
    actors: Arc<RwLock<HashMap<A::Id, ActorNode<A>>>>,
    nodes: NodesRegistry,
    arbiter: ArbiterHandle,
    factory: A::Factory,
}

impl<A: RemoteActor> Clone for ActorRegistry<A>
{
    fn clone(&self) -> Self {
        ActorRegistry {
            name: self.name.clone(),
            actors: self.actors.clone(),
            nodes: self.nodes.clone(),
            arbiter: self.arbiter.clone(),
            factory: self.factory.clone(),
        }
    }
}

impl<A: RemoteActor> ActorRegistry<A>
{
    pub fn new(nodes: NodesRegistry, factory: A::Factory) -> ActorRegistry<A> {
        ActorRegistry{
            nodes,
            name: A::name().to_owned(),
            actors: Arc::new(RwLock::new(HashMap::new())),
            arbiter: Arbiter::new().handle(),
            factory,
        }
    }
    
    pub async fn get_node(&mut self, id: A::Id) -> Result<ActorNode<A>, ActorRegistryError> {
        let rw = self.actors.clone();
        tracing::info!("Capture lock {}", id);
        let nodes_guard = rw.read().await;
        let n = Self::extract_node(nodes_guard.get(&id));
        drop(nodes_guard);
        tracing::info!("Release lock {}", id);

        match n {
            Some(a) => Ok(a),
            None => {
                let act = self.activate(id.clone());
                let node = self.add_local_node(id, act).await?;
                Ok(node)
            }
        }
    }

    pub async fn get_members(&self) -> HashSet<A::Id> {
        let rw = self.actors.clone();
        let nodes_guard = rw.read().await;
        let nodes = nodes_guard.borrow();

        let mut set = HashSet::new();
        for key in nodes.keys() {
            set.insert((*key).clone());
        }
        set
    }
    
    pub fn stop(&self) -> bool {
        self.arbiter.stop()
    }

    fn extract_node(node: Option<&ActorNode<A>>) -> Option<ActorNode<A>> {
        match node {
            Some(n) => Some(n.clone()),
            None => None,
        }
    }
    
    async fn add_local_node(
        &mut self,
        node_id: A::Id,
        actor: Addr<A>,
    ) -> Result<ActorNode<A>, ActorRegistryError> {
        let rw = self.actors.clone();
        let mut nodes_guard = rw.write().await;
        let nodes = nodes_guard.borrow_mut();

        let node = ActorNode::Local(actor);
        match nodes.insert(node_id.clone(), node.clone()) {
            None => Ok(node),
            Some(_) => {
                tracing::info!("Replace old value for {}", node_id.clone());
                Ok(node)
            }
        }
    }
    
    fn activate(&self, id: A::Id) -> Addr<A> {
        let f = self.factory.clone();
        A::start_in_arbiter(&self.arbiter, move |ctx| f.create(id, ctx))
    }
}
