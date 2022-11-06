use actix::{dev::ToEnvelope, prelude::*, WeakAddr};

use serde::{Serialize, de::DeserializeOwned};

use crate::housekeeping::{HousekeepingActor, RefreshUsage};

use super::{error::{VirtualActorSendError, SendErrorWrapper}, virtual_actor::VirtualActor, virtual_message::VirtualMessage};

enum VirtualAddrState<V: VirtualActor> {
    Local{
        id: V::Id,
        local_addr: WeakAddr<V>,
        reg: Addr<HousekeepingActor<V>>
    },
}

pub struct VirtualAddr<V: VirtualActor> {
    state: VirtualAddrState<V>,
}

impl<V: VirtualActor> VirtualAddr<V> {
    /// Sends a message unconditionally, ignoring any potential errors.
    #[allow(dead_code)]
    pub async fn do_send<M>(&self, msg: M)
    where
        M: VirtualMessage + Send,
        M::Result: Send + Serialize + DeserializeOwned,
        V: Handler<M>,
        V::Context: ToEnvelope<V, M>,
    {
        match &self.state {
            VirtualAddrState::Local{id, local_addr, reg} => {
                if let Some(addr) = local_addr.upgrade() {
                    addr.do_send(msg);
                    reg.do_send(RefreshUsage::<V>::new(&id))
                }
            }
        }
    }

    /// Tries to send a message.
    #[allow(dead_code)]
    pub async fn try_send<M>(&self, msg: M) -> Result<(), VirtualActorSendError>
    where
        M: VirtualMessage + Send + 'static,
        M::Result: Send + Serialize + DeserializeOwned,
        V: Handler<M>,
        V::Context: ToEnvelope<V, M>,
    {
        match &self.state {
            VirtualAddrState::Local{id, local_addr, reg} =>  {
                let addr = local_addr.upgrade().ok_or(VirtualActorSendError::MissingLocalActor)?;
                addr
                .try_send(msg)
                .map_err(|e| VirtualActorSendError::SendError(SendErrorWrapper::from_send_error(e)))?;
                reg.do_send(RefreshUsage::<V>::new(&id));
                Ok(())    
            }
        }
    }

    /// Sends an asynchronous message and waits for a response.
    ///
    /// The communication channel to the actor is bounded. If the
    /// returned `Future` object gets dropped, the message is
    /// cancelled.
    pub async fn send<M>(&self, msg: M) -> Result<M::Result, VirtualActorSendError>
    where
        M: VirtualMessage + Send + 'static,
        M::Result: Send + Serialize + DeserializeOwned,
        V: Handler<M>,
        V::Context: ToEnvelope<V, M>,
    {
        match &self.state {
            VirtualAddrState::Local{id, local_addr, reg} =>  {
                let addr = local_addr.upgrade().ok_or(VirtualActorSendError::MissingLocalActor)?;
                let res = addr.send(msg).await.map_err(|e| VirtualActorSendError::MailboxError(e))?;
                reg.do_send(RefreshUsage::<V>::new(&id));
                Ok(res)
            }
        }
    }
}


pub struct VirtualAddrFactory<V: VirtualActor> {
    reg: Addr<HousekeepingActor<V>>
}

impl<V: VirtualActor> VirtualAddrFactory<V> {

    pub fn new() -> VirtualAddrFactory<V> {
        VirtualAddrFactory { reg: HousekeepingActor::<V>::from_registry() }
    }

    pub fn create_from_local(&self, id: &V::Id, local_addr: WeakAddr<V>) -> VirtualAddr<V> {
        VirtualAddr{
            state: VirtualAddrState::Local { id: id.clone(), local_addr, reg: self.reg.clone() }
        }
    }
}