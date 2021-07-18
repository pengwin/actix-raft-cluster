use actix::Message;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub use remote_actor_derive::RemoteMessage;

pub trait RemoteMessage: Message + Serialize + DeserializeOwned + Send + 'static {
    fn name() -> &'static str;
}

pub trait RemoteMessageResponse: Serialize + DeserializeOwned + Send {}

impl<T, E> RemoteMessageResponse for Result<T, E>
where
    T: Serialize + DeserializeOwned + Send + 'static,
    E: Serialize + DeserializeOwned + Send + 'static,
{
}
