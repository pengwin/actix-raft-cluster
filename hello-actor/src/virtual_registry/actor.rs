use std::{collections::HashMap, sync::Arc};

use actix::prelude::*;

use thiserror::Error;

use super::error::VirtualActorRegistryError;
use crate::{
    housekeeping::{HousekeepingActor, RefreshUsage},
    virtual_actor::{
        StopRequest, StopRequestError, VirtualActor, VirtualActorFactory, VirtualAddr,
        VirtualAddrFactory,
    },
};

#[derive(Error, Debug)]
enum ParkVirtualActorError {
    #[error("Mailbox Error ({0:?}): '{0}'")]
    MailboxError(#[from] MailboxError),
    #[error("StopRequestError ({0:?}): '{0}'")]
    StopRequestError(#[from] StopRequestError),
}

pub struct VirtualActorRegistryActor<V: VirtualActor> {
    map: HashMap<V::Id, Addr<V>>,
    factory: Option<Arc<V::Factory>>,
    addr_factory: VirtualAddrFactory<V>,
}

impl<V: VirtualActor> VirtualActorRegistryActor<V> {
    fn wrap_in_future_response<T: 'static, E: 'static>(&self, r: Result<T, E>) -> ResponseActFuture<Self, Result<T, E>> {
        Box::pin(fut::result::<T, E>(r))
    }

    fn create_actor_from_factory(
        &self,
        factory: Arc<V::Factory>,
        id: V::Id,
    ) -> ResponseActFuture<Self, Result<GetActorResult<V>, VirtualActorRegistryError>> {
        let fut = async move {
            factory.create(id.clone()).await
        }.into_actor(self).map(|r, this, _| {
            let a = r.map_err(VirtualActorRegistryError::from)?;
            let actor_id = a.id();
            let addr = a.start();
            let local_addr = addr.downgrade();
            this.map.insert(actor_id.clone(), addr);
            Ok(GetActorResult {
                addr: this.addr_factory.create_from_local(&actor_id, local_addr),
            })
        });
        Box::pin(fut)
    }

    fn create_actor(
        &self,
        id: V::Id,
    ) -> ResponseActFuture<Self, Result<GetActorResult<V>, VirtualActorRegistryError>> {
        let factory = self.factory.as_ref();

        match factory {
            Some(f) => self.create_actor_from_factory(f.clone(), id),
            None => self.wrap_in_future_response(Err(VirtualActorRegistryError::FactoryIsNotSet)),
        }
    }

    fn get_or_create_actor(
        &mut self,
        id: V::Id,
    ) -> ResponseActFuture<Self, Result<GetActorResult<V>, VirtualActorRegistryError>> {
        let val = self.map.get(&id);
        match val {
            Some(addr) => {
                let res = GetActorResult {
                    addr: self.addr_factory.create_from_local(&id, addr.downgrade()),
                };
                self.wrap_in_future_response(Ok(res))
            }
            None => self.create_actor(id),
        }
    }

    async fn request_actor_to_stop(addr: Option<Addr<V>>) -> Result<(), ParkVirtualActorError> {
        match addr {
            Some(addr) => {
                let r = addr.send(StopRequest {}).await??;
                Ok(r)
            }
            None => Ok(()),
        }
    }
}

impl<V> Default for VirtualActorRegistryActor<V>
where
    V: VirtualActor,
{
    fn default() -> Self {
        Self {
            map: Default::default(),
            factory: None,
            addr_factory: VirtualAddrFactory::<V>::new(),
        }
    }
}

impl<V> Actor for VirtualActorRegistryActor<V>
where
    V: VirtualActor,
{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Actor Registry for {} started", V::name());
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Actor Registry for {} stopped", V::name());
    }
}

impl<V> actix::Supervised for VirtualActorRegistryActor<V> where V: VirtualActor {}

impl<V> SystemService for VirtualActorRegistryActor<V>
where
    V: VirtualActor,
{
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        println!("Service Registry for {} started", V::name());
    }
}

pub struct GetActorResult<V: VirtualActor> {
    pub addr: VirtualAddr<V>,
}

#[derive(Message)]
#[rtype(result = "Result<GetActorResult<V>, VirtualActorRegistryError>")]
pub struct GetActor<V: VirtualActor> {
    id: V::Id,
}

impl<V: VirtualActor> GetActor<V> {
    pub fn new(id: V::Id) -> GetActor<V> {
        GetActor { id }
    }
}

impl<V> Handler<GetActor<V>> for VirtualActorRegistryActor<V>
where
    V: VirtualActor,
{
    type Result = AtomicResponse<Self, Result<GetActorResult<V>, VirtualActorRegistryError>>;

    fn handle(&mut self, msg: GetActor<V>, _ctx: &mut Context<Self>) -> Self::Result {
        AtomicResponse::new(self.get_or_create_actor(msg.id))
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), VirtualActorRegistryError>")]
pub struct SetFactory<V: VirtualActor> {
    pub factory: V::Factory,
}

impl<V: VirtualActor> SetFactory<V> {
    pub fn new(factory: V::Factory) -> Self {
        Self { factory }
    }
}

impl<V> Handler<SetFactory<V>> for VirtualActorRegistryActor<V>
where
    V: VirtualActor,
{
    type Result = Result<(), VirtualActorRegistryError>;

    fn handle(&mut self, msg: SetFactory<V>, _ctx: &mut Context<Self>) -> Self::Result {
        self.factory = Some(Arc::new(msg.factory));
        println!("Factory for {} is set", V::name());
        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ParkVirtualActor<V: VirtualActor> {
    id: V::Id,
}

impl<V: VirtualActor> ParkVirtualActor<V> {
    pub fn new(id: &V::Id) -> ParkVirtualActor<V> {
        ParkVirtualActor { id: id.clone() }
    }
}

impl<V> Handler<ParkVirtualActor<V>> for VirtualActorRegistryActor<V>
where
    V: VirtualActor,
{
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: ParkVirtualActor<V>, _ctx: &mut Context<Self>) -> Self::Result {
        let addr_from_map = self.map.get(&msg.id).map(|a| a.to_owned());
        Box::pin(
            async move { Self::request_actor_to_stop(addr_from_map).await }
                .into_actor(self) // converts future to ActorFuture
                .map(move |res, act, _ctx| {
                    match res {
                        Ok(_) => {
                            act.map.remove(&msg.id);
                        }
                        Err(e) => {
                            println!("Unable to park actor {} {} {:?}", V::name(), msg.id, e);
                            HousekeepingActor::<V>::from_registry()
                                .do_send(RefreshUsage::new(&msg.id)); // refresh usage, reschedule actor park
                        }
                    }
                }),
        )
    }
}
