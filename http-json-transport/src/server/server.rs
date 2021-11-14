use std::time::Duration;

use actix_web::middleware::Logger;
use actix_web::{get, dev::Server, web, App, HttpResponse, HttpServer};

use super::error::ServerError;
use crate::config::Config;
//use std::sync::Arc;

struct AppState {
    name: String,
}

#[get("/healthcheck")]
async fn health_check(data: web::Data<AppState>) -> String {
    data.name.clone()
}

pub fn create_server(
    config: &Config,
    sever_workers_number: u8,
    shutdown_timeout: Duration,
    //registry: Arc<RegistryCollection>,
) -> Result<Server, ServerError> {
    if !actix_rt::System::is_registered() {
        return Err(ServerError::ThreadDoesntHaveSystem);
    }

    //let configurator = Arc::new(NodeActorWebConfigurator::new(registry.node_actors_factory()));
    let bind_point = config.server_endpoint.to_owned();

    tracing::info!("Binding server to {}", bind_point);

    let name = config.name.to_owned();

    let srv = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            //.configure(|s| configurator.clone().config(s))
            .app_data(web::Data::new(AppState { name: name.clone() }))
            .service(health_check)
            .route(".../{tail}*", web::get().to(HttpResponse::NotFound))
    })
    .bind(bind_point)
    .map_err(ServerError::from)?
    .shutdown_timeout(shutdown_timeout.as_secs())
    .workers(sever_workers_number.into())
    .run();

    Ok(srv)
}
