use actix::{Addr, Arbiter, ArbiterHandle};
use remote_actor::{RemoteActor, RemoteActorAddr, RemoteActorFactory};
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{ActorNode, ActorRegistryError};
use crate::{ActorFromNodes, NodesRegistry, NodesRegistryFactory};

pub struct ActorRegistryFactory<A: RemoteActor> {
    nodes_registry_factory: Arc<NodesRegistryFactory>,
    arbiter: ArbiterHandle,
    factory: Arc<A::Factory>,
    actors: Arc<RwLock<HashMap<A::Id, ActorNode<A>>>>,
}

impl<A: RemoteActor> ActorRegistryFactory<A> {
    pub fn new(nodes_registry_factory: Arc<NodesRegistryFactory>, factory: A::Factory) -> ActorRegistryFactory<A> {
        ActorRegistryFactory {
            nodes_registry_factory,
            arbiter: Arbiter::new().handle(),
            factory: Arc::new(factory),
            actors: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn create(&self) -> ActorRegistry<A> {
        ActorRegistry::<A>::new(self.nodes_registry_factory.create(), self.arbiter.clone(), self.factory.clone(), self.actors.clone(),)
    }

    pub fn stop(&self) -> bool {
        self.arbiter.stop()
    }
}

pub struct ActorRegistry<A: RemoteActor> {
    nodes: NodesRegistry,
    actors: Arc<RwLock<HashMap<A::Id, ActorNode<A>>>>,
    arbiter: ArbiterHandle,
    factory: Arc<A::Factory>,
}

impl<A: RemoteActor> ActorRegistry<A> {
    pub fn new(nodes: NodesRegistry, arbiter: ArbiterHandle, factory: Arc<A::Factory>, actors: Arc<RwLock<HashMap<A::Id, ActorNode<A>>>>) -> ActorRegistry<A> {
        ActorRegistry {
            nodes,
            actors,
            arbiter,
            factory,
        }
    }

    pub async fn get_or_activate_node(
        &self,
        id: A::Id,
    ) -> Result<ActorNode<A>, ActorRegistryError> {
        let rw = self.actors.clone();
        tracing::info!("Capture lock {}", id);
        let nodes_guard = rw.read().await;
        let n = Self::extract_node(nodes_guard.get(&id));
        drop(nodes_guard);
        tracing::info!("Release lock {}", id);

        match n {
            Some(a) => Ok(a),
            None => match self.nodes.get_actor::<A>(id.clone()).await {
                ActorFromNodes::Remote(r) => {
                    let node = self.add_remote_node(id, r).await?;
                    Ok(node)
                }
                ActorFromNodes::Local => {
                    let act = self.activate(id.clone());
                    let node = self.add_local_node(id, act).await?;

                    Ok(node)
                }
                ActorFromNodes::NotFound => Err(ActorRegistryError::NodeNotFound),
            },
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
    
    fn extract_node(node: Option<&ActorNode<A>>) -> Option<ActorNode<A>> {
        node.cloned()
    }

    async fn add_local_node(
        &self,
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

    async fn add_remote_node(
        &self,
        node_id: A::Id,
        actor: RemoteActorAddr<A>,
    ) -> Result<ActorNode<A>, ActorRegistryError> {
        let rw = self.actors.clone();
        let mut nodes_guard = rw.write().await;
        let nodes = nodes_guard.borrow_mut();

        let node = ActorNode::Remote(actor);
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
