use actix::prelude::*;
use actix_web::{App, HttpServer};
use handler::config;
use second_remote_actor::{SecondRemoteActor, SecondRemoteActorFactory, RemoteRequest};
use test_remote_actor::{TestRemoteActor, TestRemoteActorFactory};
use tracing_actix_web::TracingLogger;
use virtual_registry::SetFactory;

mod handler;
mod test_remote_actor;
mod second_remote_actor;
mod virtual_actor;
mod virtual_registry;
mod housekeeping;
mod inmemory_state_persistence;
mod file_state_persistence;
mod persistence;

use crate::{test_remote_actor::Ping, virtual_registry::VirtualActorRegistryActor};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let reg = VirtualActorRegistryActor::<TestRemoteActor>::from_registry();
    let _ = reg.send(SetFactory::new(TestRemoteActorFactory::new())).await;

    let reg = VirtualActorRegistryActor::<SecondRemoteActor>::from_registry();
    let _ = reg.send(SetFactory::new(SecondRemoteActorFactory::new())).await;

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .configure(config::<TestRemoteActor, Ping>)
            .configure(config::<SecondRemoteActor, RemoteRequest>)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
