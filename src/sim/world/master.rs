//! The world state is a collection of components that connect to each other.
//!
//! The world state responds to messages defined in sim::requests
use crate::{
    common::world::{ComponentId, ComponentIdIncrementer, GateInputSocket, GateOutputSocket},
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
        self.gates.tick_all()
    }

    /// get data at output socket
    pub fn get_buffer(&self, output_socket: &GateOutputSocket) -> Option<&SimData> {
        self.gates.get_output(output_socket)
    }

    pub fn connect_gates(
        &mut self,
        output_socket: GateOutputSocket,
        input_socket: GateInputSocket,
    ) -> Result<(), Box<sim::Error>> {
        self.gates.connect(output_socket, input_socket)
    }
}
