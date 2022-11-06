use std::{fmt::Display, hash::Hash};


use actix::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

use super::{VirtualActorFactory, StoppableVirtualActor};

pub trait VirtualActor: Actor<Context = Context<Self>> + StoppableVirtualActor {
    type Factory: VirtualActorFactory<Self>;
    type Id: Serialize + DeserializeOwned + Unpin + Eq + Hash + Clone + Send + Display;
    fn name() -> &'static str;

    fn id(&self) -> Self::Id;
}



