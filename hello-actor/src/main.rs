mod file_state_persistence;
mod handler;
mod housekeeping;
mod inmemory_state_persistence;
mod persistence;
mod second_remote_actor;
mod test_remote_actor;
mod virtual_actor;
mod virtual_registry;


use actix::prelude::*;
use actix_web::{App, HttpServer};
use handler::config;
use second_remote_actor::{RemoteRequest, SecondRemoteActor, SecondRemoteActorFactory};
use test_remote_actor::{TestRemoteActor, TestRemoteActorFactory};
use tracing_actix_web::TracingLogger;
use virtual_registry::{SetFactory, StopAllActors};


use crate::{test_remote_actor::Ping, virtual_registry::VirtualActorRegistryActor};

use va_tracing::{setup_tracing, ConsoleConfig, TracingConfig, TracingError};

fn main() -> std::io::Result<()> {
    setup_tracing(TracingConfig {
        console_config: Some(ConsoleConfig {
            enabled: false,
            pretty_print: true,
        }),
        jaeger_config: None,
    })
    .map_err(to_io)?;

    let span = tracing::span!(tracing::Level::INFO, "Main");
    let _enter = span.enter();

    tracing::info!("Tracing initialized");

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
                match reg.send(StopAllActors {}).await {
                    Ok(r) => match r {
                        Ok(_) => tracing::debug!("Stopped all on signal"),
                        Err(e) => tracing::error!("Unable to stop all on signal {}", e),
                    },
                    Err(e) => tracing::error!("Unable to send on signal {}", e),
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

fn to_io(e: TracingError) -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Tracing Init Error {:?}", e),
    )
}
