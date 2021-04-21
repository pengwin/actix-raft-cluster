use actix_web::{web, HttpResponse, Result as ActixResult};
use serde::{Deserialize};
use actix::Handler;
use std::marker::PhantomData;
use actix::dev::ToEnvelope;

use actor_registry::ActorRegistry;
use remote_actor::{actor_url_template, RemoteMessageResponse};
use remote_actor::{RemoteActor, RemoteMessage};


pub struct AppStateWithRegistry<A: RemoteActor> {
    pub registry: ActorRegistry<A>,
}

impl<A> AppStateWithRegistry<A> where
    A: RemoteActor
{
    pub fn new(registry: ActorRegistry<A>) -> AppStateWithRegistry<A> {
        AppStateWithRegistry{registry}
    }
}

pub struct Configurator<A> where A: RemoteActor {
    phantom_a: PhantomData<A>,
}

#[derive(Deserialize)]
pub struct ActorIdPath<I> {
    id: I
}

impl<A: RemoteActor> Configurator<A> {
    async fn send<M>(data: web::Data<AppStateWithRegistry<A>>, actor_id: web::Path<ActorIdPath<A::Id>>, json: web::Json<M>) -> ActixResult<HttpResponse> where
        A: RemoteActor + Handler<M>,
        M: RemoteMessage,
        M::Result: RemoteMessageResponse,
        A::Context: ToEnvelope<A, M> {
        tracing::info!("Begin handler {}/{}", A::name(), M::name());
        let id = actor_id.id.clone();
        
        let r = data.registry.clone().get_node(id).await;
        let request = json.into_inner();
        match r {
            Ok(actor) => {
                tracing::info!("Actor received");
                let actor_response = actor.send(request).await;
                return match actor_response {
                    Ok(s) => {
                        let p = serde_json::to_string(&s)?;
                        tracing::info!("Actor result {}", p);
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

    pub fn config_message<M>(cfg: &mut web::ServiceConfig) where
        A: RemoteActor + Handler<M>,
        M: RemoteMessage,
        M::Result: RemoteMessageResponse,
        A::Context: ToEnvelope<A, M> {
        let path = actor_url_template::<A, M>();
        cfg.route(&path, web::post().to(Self::send::<M>));
    }
}

