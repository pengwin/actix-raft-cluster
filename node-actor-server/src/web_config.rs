use actix_web::{web, HttpResponse, Result as ActixResult};
use remote_actor_server::{AppStateWithRegistry, Configurator};

use node_actor::{NodeActor, AttachNode, Metrics, NodeActorRegistry};

#[derive(Clone)]
pub struct NodeActorWebConfigurator {
    state: web::Data<AppStateWithRegistry<NodeActor>>
}

impl NodeActorWebConfigurator {
    
    pub fn new(registry: NodeActorRegistry) -> NodeActorWebConfigurator {
        NodeActorWebConfigurator {
            state: web::Data::new(AppStateWithRegistry::new(registry))
        }
    }

    async fn health(data: web::Data<AppStateWithRegistry<NodeActor>>) -> ActixResult<HttpResponse>{
        let id = 1u64;

        let r = data.registry.clone().get_node(id).await;
        match r {
            Ok(actor) => {
                let actor_response = actor.send(Metrics{}).await;
                return match actor_response {
                    Ok(s) => {
                        Ok(HttpResponse::Ok().json(s))
                    },
                    Err(e) => {
                        tracing::error!("Actor Send Error {}", e);
                        Ok(HttpResponse::InternalServerError().body(format!("Actor Error {}", e)))
                    }
                };
            },
            Err(e) => {
                Ok(HttpResponse::BadRequest().body(format!("Actor registry {} error", e)))
            }
        }
    }
    
    pub fn config(&self, cfg: &mut web::ServiceConfig) {
        let state = self.state.clone();
        cfg.app_data(state);
        Configurator::<NodeActor>::config_message::<AttachNode>(cfg);
        Configurator::<NodeActor>::config_message::<Metrics>(cfg);
        cfg.route("/health", web::get().to(Self::health));
    }
}

