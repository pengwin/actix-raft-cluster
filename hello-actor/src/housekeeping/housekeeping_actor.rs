use std::collections::HashMap;
use std::time::Instant;

use actix::prelude::*;
use std::time::Duration;

use crate::virtual_registry::{ParkVirtualActor, VirtualActorRegistryActor};

use crate::virtual_actor::VirtualActor;

pub struct ActorStatistics {
    last_used: Instant,
}

impl Default for ActorStatistics {
    fn default() -> Self {
        Self {
            last_used: Instant::now(),
        }
    }
}

pub struct HousekeepingActor<V: VirtualActor> {
    map: HashMap<V::Id, ActorStatistics>,
    reg: Addr<VirtualActorRegistryActor<V>>,
}

impl<V> Default for HousekeepingActor<V>
where
    V: VirtualActor,
{
    fn default() -> Self {
        Self {
            map: Default::default(),
            reg: VirtualActorRegistryActor::<V>::from_registry(),
        }
    }
}

impl<V> Actor for HousekeepingActor<V>
where
    V: VirtualActor,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Service HousekeepingActor actor started");
        ctx.run_interval(Duration::from_secs(1), |_, c| {
            c.address().do_send(RevisitUsage {
                lifetime_threshold: Duration::from_secs(20),
            })
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Service HousekeepingActor actor stopped");
    }
}

impl<V> actix::Supervised for HousekeepingActor<V> where V: VirtualActor {}

impl<V> SystemService for HousekeepingActor<V>
where
    V: VirtualActor,
{
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        println!("Service started");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RefreshUsage<V: VirtualActor> {
    pub id: V::Id,
}

impl<V: VirtualActor> RefreshUsage<V> {
    pub fn new(id: &V::Id) -> Self {
        Self { id: id.clone() }
    }
}

impl<V> Handler<RefreshUsage<V>> for HousekeepingActor<V>
where
    V: VirtualActor,
{
    type Result = ();

    fn handle(&mut self, msg: RefreshUsage<V>, _ctx: &mut Context<Self>) -> Self::Result {
        self.map
            .entry(msg.id)
            .and_modify(|s| s.last_used = Instant::now())
            .or_insert(ActorStatistics::default());
    }
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct RevisitUsage {
    lifetime_threshold: Duration,
}

impl<V> Handler<RevisitUsage> for HousekeepingActor<V>
where
    V: VirtualActor,
{
    type Result = ();

    fn handle(&mut self, msg: RevisitUsage, ctx: &mut Context<Self>) -> Self::Result {
        for (id, stat) in &self.map {
            if stat.last_used.elapsed() >= msg.lifetime_threshold {
                ctx.address().do_send(UnregisterActor::<V>::new(id));
            }
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct UnregisterActor<V: VirtualActor> {
    id: V::Id,
}

impl<V: VirtualActor> UnregisterActor<V> {
    pub fn new(id: &V::Id) -> UnregisterActor<V> {
        UnregisterActor { id: id.clone() }
    }
}

impl<V> Handler<UnregisterActor<V>> for HousekeepingActor<V>
where
    V: VirtualActor,
{
    type Result = ();

    fn handle(&mut self, msg: UnregisterActor<V>, _ctx: &mut Context<Self>) -> Self::Result {
        self.reg.do_send(ParkVirtualActor::<V>::new(&msg.id));
        self.map.remove(&msg.id);
        println!("Stop actor {}", msg.id);
    }
}
