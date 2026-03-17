use std::sync::mpsc;

use tokio::sync::oneshot;

use crate::{
    common::world::ComponentId,
    world::{MasterWorld, user::ConnectRequest},
};

/// server side world state
pub struct ServerWorld {
    world: MasterWorld,
    rx: mpsc::Receiver<WorldRequest>,
}

/// response when a server world is created
pub struct CreatedServerWorld {
    pub world: ServerWorld,
    pub sender: mpsc::Sender<WorldRequest>,
}

impl ServerWorld {
    /// turn a MasterWorld into a server world
    pub fn with_world(world: MasterWorld) -> CreatedServerWorld {
        let (tx, rx) = mpsc::channel();
        CreatedServerWorld {
            world: Self { world, rx },
            sender: tx,
        }
    }

    /// start the world as a server
    pub fn run_blocking(&mut self) {
        while let Ok(request) = self.rx.recv() {
            match request {
                WorldRequest::PlayerConnect { body, res } => {
                    let _ = res.send(match self.world.connect(body) {
                        Ok(id) => responses::PlayerConnect::Accepted(id),
                        Err(e) => responses::PlayerConnect::Rejected(e),
                    });
                }
            }
        }
    }
}

/// requests to send to the server
pub enum WorldRequest {
    PlayerConnect {
        /// request body
        body: ConnectRequest,
        /// response hook
        res: oneshot::Sender<responses::PlayerConnect>,
    },
}

/// responses from the server
pub mod responses {
    use crate::{common::world::ComponentId, world::WorldError};

    pub enum PlayerConnect {
        /// connection accepted, and returns the user id of the player
        Accepted(ComponentId),
        Rejected(Box<WorldError>),
    }
}
