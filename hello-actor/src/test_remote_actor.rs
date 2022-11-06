use actix::prelude::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use thiserror::Error;

use crate::{
    inmemory_state_persistence::{InmemoryStatePersistence, InmemoryStatePersistenceFactory},
    persistence::{PersistentState, VirtualActorState, VirtualActorWithState},
    virtual_actor::{
        StopRequest, StopRequestError, StoppableVirtualActor, VirtualActor, VirtualActorFactory,
        VirtualActorFactoryError, VirtualMessage,
    },
};

#[derive(Default, Serialize, Deserialize)]
pub struct TestRemoteState {
    counter: u8,
}

impl VirtualActorState for TestRemoteState {}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum TestRemoteActorError {
    #[error("SomeError")]
    SomeError,
}

pub struct TestRemoteActorFactory {
    persistence_factory: InmemoryStatePersistenceFactory<TestRemoteActor>,
}

impl TestRemoteActorFactory {
    pub fn new() -> Self {
        Self {
            persistence_factory: InmemoryStatePersistenceFactory::new(),
        }
    }
}

#[async_trait]
impl VirtualActorFactory<TestRemoteActor> for TestRemoteActorFactory {
    async fn create(&self, id: String) -> Result<TestRemoteActor, VirtualActorFactoryError> {
        let persistence = self.persistence_factory.create();
        let mut state = PersistentState::new(id.clone(), persistence);
        state.load().await?;
        Ok(TestRemoteActor { state })
    }
}

pub struct TestRemoteActor {
    state: PersistentState<Self>,
}

impl VirtualActor for TestRemoteActor {
    type Factory = TestRemoteActorFactory;
    type Id = String;

    fn id(&self) -> Self::Id {
        self.state.id()
    }

    fn name() -> &'static str {
        "TestRemoteActor"
    }
}

impl Actor for TestRemoteActor {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Started {}", self.id())
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Sopped {}", self.id())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Res {
    pub p: String,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<Res, String>")]
pub struct Ping {
    pub message: String,
}

impl VirtualMessage for Ping {
    fn name() -> &'static str {
        "Ping"
    }
}

impl Handler<Ping> for TestRemoteActor {
    type Result = Result<Res, String>;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        println!("ping {}", msg.message);
        if msg.message == "Error" {
            return Err("Error".to_owned());
        }

        self.state.modify(|s| s.counter += 1);
        let counter = self.state.select(|s| s.counter);

        Ok(Res {
            p: format!("{} {}", msg.message, counter),
        })
    }
}

#[async_trait]
impl StoppableVirtualActor for TestRemoteActor {
    /*async fn stop() -> Result<(), StopRequestError> {
        self.
    }*/
}

impl Handler<StopRequest> for TestRemoteActor {
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

impl VirtualActorWithState for TestRemoteActor {
    type State = TestRemoteState;

    type StatePersistence = InmemoryStatePersistence<Self>;
}
