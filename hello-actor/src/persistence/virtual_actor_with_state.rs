use crate::virtual_actor::VirtualActor;

use super::{VirtualActorState, StatePersistence};

pub trait VirtualActorWithState: VirtualActor {
    type State: VirtualActorState;
    type StatePersistence: StatePersistence<Self>;
}
