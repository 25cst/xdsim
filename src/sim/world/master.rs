use semver::Version;

use crate::{
    common::world::{ComponentId, ComponentIdIncrementer},
    sim::{
        self,
        requests::{CreateBlankWorld, CreateDefaultGate, RegisterNewGateOutputByIndex},
        world::{data::WorldStateData, gates::WorldStateGates},
    },
};

pub type PackageName = String;
pub type PackageVersion = Version;
pub type ComponentName = String;

/// The representation for simulation state
pub struct WorldState {
    data: WorldStateData,
    gates: WorldStateGates,
    id_counter: ComponentIdIncrementer,
}

impl WorldState {
    /// create new empty world with library handles to gates and data
    pub fn new_blank(request: CreateBlankWorld) -> Self {
        Self {
            data: WorldStateData::new_blank(request.data_handles),
            gates: WorldStateGates::new_blank(request.gate_handles),
            id_counter: ComponentIdIncrementer::zero(),
        }
    }

    /// Create a new gate in world with default state
    pub fn create_default_gate(
        &mut self,
        request: CreateDefaultGate,
    ) -> Result<ComponentId, Box<sim::Error>> {
        self.gates
            .create_default_gate(request.gate, &self.data, &mut self.id_counter)
    }

    /// - Registers a new output (thus creating a never-before-existed buffer)
    /// - By index: the index of the output in the definition array
    pub fn register_new_gate_output_by_index(
        &mut self,
        request: RegisterNewGateOutputByIndex,
    ) -> Result<ComponentId, Box<sim::Error>> {
        self.gates.register_new_output_by_index(
            &mut self.data,
            request.gate_output_socket,
            &mut self.id_counter,
        )
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
    pub fn tick_all(&mut self) -> Result<(), Box<sim::Error>> {
        let res = self.gates.tick_all(&mut self.data);
        self.data.flush();
        res
    }
}
