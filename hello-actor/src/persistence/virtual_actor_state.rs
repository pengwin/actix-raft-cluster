use serde::{de::DeserializeOwned, Serialize};

pub trait VirtualActorState: Default + Serialize + DeserializeOwned {
}