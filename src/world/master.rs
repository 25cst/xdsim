use crate::{
    common::world::ComponentId,
    world::user::{self, ConnectRequest},
};

pub type WorldError = user::Error;

pub struct MasterWorld {
    world: user::WorldState,
}

impl MasterWorld {
    pub fn connect(&mut self, request: ConnectRequest) -> Result<ComponentId, Box<WorldError>> {
        self.world.connect(request)
    }

    pub fn disconnect(&mut self, id: &ComponentId) -> Result<(), Box<WorldError>> {
        self.world.disconnect(id)
    }
}
