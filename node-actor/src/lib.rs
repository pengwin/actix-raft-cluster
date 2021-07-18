mod actor;
mod attach_node;
mod error;
mod init;
mod metrics;
mod ping;

use actor_registry::{ActorRegistry, ActorRegistryFactory};
use remote_actor::RemoteActorAddr;

pub type NodeActorId = u64;

pub type RemoteNodeActorAddr = RemoteActorAddr<NodeActor>;
pub type NodeActorRegistry = ActorRegistry<NodeActor>;
pub type NodeActorRegistryFactory = ActorRegistryFactory<NodeActor>;

pub use actor::*;
pub use attach_node::*;
pub use error::*;
pub use init::*;
pub use metrics::*;
pub use ping::*;
