mod actor;
mod init;
mod attach_node;
mod error;
mod metrics;

use remote_actor::RemoteActorAddr;
use actor_registry::ActorRegistry;

pub type NodeActorId = u64;

pub type RemoteNodeActorAddr = RemoteActorAddr<NodeActor>;
pub type NodeActorRegistry = ActorRegistry<NodeActor>;

pub use actor::*;
pub use init::*;
pub use attach_node::*;
pub use metrics::*;
pub use error::*;
