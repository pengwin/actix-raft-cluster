use actix::Message;
use serde::{de::DeserializeOwned, Serialize};

pub trait VirtualMessage: Message + Serialize + DeserializeOwned
where
    <Self as Message>::Result: Serialize + DeserializeOwned,
{
    fn name() -> &'static str;
}
