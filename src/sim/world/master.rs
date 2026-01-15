//! The world state is a collection of components that connect to each other.
//!
//! The world state responds to messages defined in sim::requests
use semver::Version;

#[cfg(test)]
use crate::common::world::DataPtr;
use crate::{
    common::world::{ComponentId, ComponentIdIncrementer},
    sim::{
        self,
        component::SimData,
        requests::*,
        world::{data::WorldStateData, gates::WorldStateGates},
    },
};

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

    /// Registers a new output (thus creating a never-before-existed buffer)
    pub fn register_new_gate_output(
        &mut self,
        request: RegisterNewGateOutput,
    ) -> Result<ComponentId, Box<sim::Error>> {
        self.gates
            .register_new_output(&mut self.data, request.socket, &mut self.id_counter)
    }

    /// Registers an output socket to output to an existing buffer
    pub fn register_existing_gate_output(
        &mut self,
        request: RegisterExistingGateOutput,
    ) -> Result<(), Box<sim::Error>> {
        self.gates
            .register_existing_output(&mut self.data, request.socket, request.buffer)
    }

    /// Registers an input socket to take input from an existing buffer
    pub fn register_existing_gate_input(
        &mut self,
        request: RegisterExistingGateInput,
    ) -> Result<(), Box<sim::Error>> {
        self.gates
            .register_existing_input(&mut self.data, request.socket, request.buffer)
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

    /// # Safety
    ///
    /// the pointer has not safety guarantees besides that it is valid, you may not modify or drop
    /// the pointer
    ///
    /// get data
    pub fn get_data(&self, buffer_id: &ComponentId) -> Option<&SimData> {
        self.data.get_data(buffer_id)
    }
}
