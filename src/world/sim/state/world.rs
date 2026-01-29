//! The world state is a collection of components that connect to each other.
//!
//! The world state responds to messages defined in sim::requests
use crate::{
    common::world::{ComponentId, ComponentIdIncrementer, GateOutputSocket},
    world::sim::{
        self, SimGate,
        component::SimData,
        requests::*,
        state::{data::WorldStateData, gates::WorldStateGates},
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

    /// get a gate by ID
    ///
    /// # Safety
    ///
    /// The gate is no longer valid after it is dropped,
    /// you should not store the reference
    pub fn get_gate(&self, gate_id: &ComponentId) -> Result<&SimGate, Box<sim::Error>> {
        match self.gates.get_gate(gate_id) {
            Some(gate) => Ok(gate),
            None => Err(sim::Error::GateNotFound { gate_id: *gate_id }.into()),
        }
    }

    /// connect an input socket to an output socket,
    /// requires: the input socket to not previously be connected to any other sockets
    pub fn connect_gates(&mut self, request: ConnectIOSockets) -> Result<(), Box<sim::Error>> {
        self.gates
            .connect(request.output_socket, request.input_socket)
    }
}
