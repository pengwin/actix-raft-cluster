use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use node_actor_server::NodeActorWebConfigurator;

use super::error::ServerError;
use crate::node::RegistryCollection;
use crate::web_server::ServerConfig;
use std::sync::Arc;

struct AppState {
    name: String,
}

async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.name;

    format!("Cluster Node. {}!", app_name)
}

pub fn create_server(
    server_config: &ServerConfig,
    registry: Arc<RegistryCollection>,
) -> Result<Server, ServerError> {
    if !actix_rt::System::is_registered() {
        return Err(ServerError::ThreadDoesntHaveSystem);
    }

    let configurator = Arc::new(NodeActorWebConfigurator::new(registry.node_actors_factory()));
    let bind_point = server_config.bind_point.to_owned();

    tracing::info!("Binding server to {}", bind_point);

    let name = server_config.name.clone();

    let srv = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(|s| configurator.clone().config(s))
            .data(AppState { name: name.clone() })
            .route("/", web::get().to(index))
            .route("*", web::get().to(HttpResponse::NotFound))
    })
    .bind(bind_point)
    .map_err(ServerError::from)?
    .shutdown_timeout(server_config.shutdown_timeout.as_secs())
    .workers(server_config.sever_workers_number)
    .run(); // <- Set shutdown timeout to 60 seconds;

    Ok(srv)
}
