use semver::Version;

use crate::sim::{
    self,
    requests::CreateBlankWorld,
    world::{data::WorldStateData, gates::WorldStateGates},
};

pub type PackageName = String;
pub type PackageVersion = Version;
pub type ComponentName = String;

/// The representation for simulation state
pub struct WorldState {
    data: WorldStateData,
    gates: WorldStateGates,
}

impl WorldState {
    /// create new empty world with library handles to gates and data
    pub fn new_blank(request: CreateBlankWorld) -> Self {
        Self {
            data: WorldStateData::new_blank(request.data_handles),
            gates: WorldStateGates::new_blank(request.gate_handles),
        }
    }

    /// tick the current world
    /// if this function returns error, its not end of the world
    /// it just means a buffer is used as input to a gate, but is not present
    /// this could be caused by bad implementation for edge cases such as:
    /// - new connection just added
    /// - an existing connection just been removed
    ///
    /// for a good implementation this should not happen.
    /// if an error is given, simply put it in debug logs or somewhere else
    pub fn tick_all(&mut self) -> Result<(), sim::Error> {
        let res = self.gates.tick_all(&mut self.data);
        self.data.flush();
        res
    }
}
