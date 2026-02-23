use crate::{
    common::world::ComponentId,
    world::{
        layout::{
            self,
            requests::{CreateBlankWorld, CreateDefaultGate},
            state::gates::WorldStateGates,
        },
        sim,
    },
};

/// layout world state: wraps around sim world state and contains layout information,
/// i.e. position of gates and conns
pub struct WorldState {
    /// wrapped simulation world
    sim_state: sim::WorldState,

    /// positions and paths of conns
    // conns: WorldStateConns,

    /// position of gates
    gates: WorldStateGates,
}

impl WorldState {
    /// create new blank layout world
    pub fn new_blank(request: CreateBlankWorld) -> Self {
        Self {
            sim_state: sim::WorldState::new_blank(sim::requests::CreateBlankWorld {
                gate_handles: request.gate_handles,
                data_handles: request.data_handles,
            }),
            gates: WorldStateGates::new_blank(),
        }
    }

    /// create a new gate in layout world with default state
    pub fn create_default_gate(
        &mut self,
        request: CreateDefaultGate,
    ) -> Result<ComponentId, Box<layout::Error>> {
        let gate_id = self
            .sim_state
            .create_default_gate(sim::requests::CreateDefaultGate { gate: request.gate })
            .map_err(layout::Error::Sim)?;

        self.gates.add_gate(
            gate_id,
            request.origin,
            self.sim_state
                .get_gate(&gate_id)
                .map_err(layout::Error::Sim)?,
        );

        Ok(gate_id)
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
    pub fn tick_all(&mut self) -> Result<(), Box<layout::Error>> {
        self.sim_state.tick_all().map_err(layout::Error::Sim)?;
        Ok(())
    }
}
