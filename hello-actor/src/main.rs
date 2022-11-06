use actix::prelude::*;
use actix_web::{App, HttpServer};
use handler::config;
use second_remote_actor::{RemoteRequest, SecondRemoteActor, SecondRemoteActorFactory};
use test_remote_actor::{TestRemoteActor, TestRemoteActorFactory};
use tracing_actix_web::TracingLogger;
use virtual_registry::{SetFactory, StopAllActors};

mod file_state_persistence;
mod handler;
mod housekeeping;
mod inmemory_state_persistence;
mod persistence;
mod second_remote_actor;
mod test_remote_actor;
mod virtual_actor;
mod virtual_registry;

use crate::{test_remote_actor::Ping, virtual_registry::VirtualActorRegistryActor};

fn main() -> std::io::Result<()> {
    let sys = actix_rt::System::new();
    return sys.block_on(async {
        let reg = VirtualActorRegistryActor::<TestRemoteActor>::from_registry();
        let _ = reg
            .send(SetFactory::new(TestRemoteActorFactory::new()))
            .await;

        let reg = VirtualActorRegistryActor::<SecondRemoteActor>::from_registry();
        let _ = reg
            .send(SetFactory::new(SecondRemoteActorFactory::new()))
            .await;

            tokio::spawn(async move {
                if let Ok(_) = tokio::signal::ctrl_c().await {
                    match reg.send(StopAllActors{}).await {
                        Ok(r) => match r {
                            Ok(_) => println!("Stopped all on signal"),
                            Err(e) => println!("Unable to stop all on signal {}", e)
                        }
                        Err(e) => println!("Unable to send on signal {}", e)
                    }
                }
            });

        HttpServer::new(|| {
            App::new()
                .wrap(TracingLogger::default())
                .configure(config::<TestRemoteActor, Ping>)
                .configure(config::<SecondRemoteActor, RemoteRequest>)
        })
        .shutdown_timeout(30)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    });
}
