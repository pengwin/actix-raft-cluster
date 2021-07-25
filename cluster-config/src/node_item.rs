use std::hash::Hash;
use std::cmp::{PartialEq, Eq};
use evmap::ShallowCopy;
use std::mem::ManuallyDrop;
use std::fmt::Display;
use remote_actor::{ActorAddr, RemoteActor, RemoteActorAddr};

/// Node Id
pub type NodeId = u64;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct NodeItem {
    pub id: NodeId,
    pub addr: ActorAddr,
}

impl ShallowCopy for NodeItem {
    unsafe fn shallow_copy(&self) -> ManuallyDrop<Self> {
        ManuallyDrop::new(Self{
            id: self.id,
            addr: self.addr.clone()
        })
    }
}

impl Display for NodeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(id {})", self.id)
    }
}

impl NodeItem {
    pub fn new(id: NodeId, addr: &str) -> NodeItem {
        NodeItem {
            id,
            addr: ActorAddr::from(addr),
        }
    }

    pub fn to_remote_actor_addr<A: RemoteActor>(&self, id: A::Id) -> RemoteActorAddr<A> {
        RemoteActorAddr::<A>::new(id, self.addr.clone())
    }
}
