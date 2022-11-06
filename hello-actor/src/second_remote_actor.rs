use actix::prelude::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use thiserror::Error;

use crate::{
    persistence::{PersistentState, VirtualActorState, VirtualActorWithState},
    virtual_actor::{
        StopRequest, StopRequestError, StoppableVirtualActor, VirtualActor, VirtualActorFactory,
        VirtualActorFactoryError, VirtualMessage,
    }, file_state_persistence::{FileStatePersistenceFactory, FileStatePersistence},
};

#[derive(Default, Serialize, Deserialize)]
pub struct SecondRemoteState {
    counter: u8,
}

impl VirtualActorState for SecondRemoteState {}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SecondRemoteActorError {
    #[error("SomeError")]
    SomeError,
}

pub struct SecondRemoteActorFactory {
    persistence_factory: FileStatePersistenceFactory<SecondRemoteActor>,
}

impl SecondRemoteActorFactory {
    pub fn new() -> Self {
        Self {
            persistence_factory: FileStatePersistenceFactory::new(),
        }
    }
}

#[async_trait]
impl VirtualActorFactory<SecondRemoteActor> for SecondRemoteActorFactory {
    async fn create(&self, id: String) -> Result<SecondRemoteActor, VirtualActorFactoryError> {
        let persistence = self.persistence_factory.create();
        let mut state = PersistentState::new(id.clone(), persistence);
        state.load().await?;
        Ok(SecondRemoteActor { state })
    }
}

pub struct SecondRemoteActor {
    state: PersistentState<Self>,
}

impl VirtualActor for SecondRemoteActor {
    type Factory = SecondRemoteActorFactory;
    type Id = String;

    fn id(&self) -> Self::Id {
        self.state.id()
    }

    fn name() -> &'static str {
        "SecondRemoteActor"
    }
}

impl Actor for SecondRemoteActor {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Started {}", self.id())
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Sopped {}", self.id())
    }
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<String, String>")]
pub struct RemoteRequest {
    pub message: String,
}

impl VirtualMessage for RemoteRequest {
    fn name() -> &'static str {
        "RemoteRequest"
    }
}

impl Handler<RemoteRequest> for SecondRemoteActor {
    type Result = Result<String, String>;

    fn handle(&mut self, msg: RemoteRequest, _ctx: &mut Context<Self>) -> Self::Result {
        println!("RemoteRequest {}", msg.message);
        if msg.message == "Error" {
            return Err("Error".to_owned());
        }

        self.state.modify(|s| s.counter += 1);
        let counter = self.state.select(|s| s.counter);

        Ok(format!("{} {}", msg.message, counter))
    }
}

#[async_trait]
impl StoppableVirtualActor for SecondRemoteActor {
    /*async fn stop() -> Result<(), StopRequestError> {
        self.
    }*/
}

impl Handler<StopRequest> for SecondRemoteActor {
    type Result = AtomicResponse<Self, Result<(), StopRequestError>>;

    fn handle(&mut self, _msg: StopRequest, _ctx: &mut Context<Self>) -> Self::Result {
        AtomicResponse::new(Box::pin(self.state.save_actor(self).map(
            |res, _act, ctx| {
                res?;
                ctx.stop();
                Ok(())
            },
        )))
    }
}

impl VirtualActorWithState for SecondRemoteActor {
    type State = SecondRemoteState;

    type StatePersistence = FileStatePersistence<Self>;
}
