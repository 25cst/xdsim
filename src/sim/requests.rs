use std::collections::HashMap;

use crate::{
    common::world::{ComponentId, ComponentVersion, GateOutputSocket},
    sim::world::{DestructedDataHandles, DestructedGateHandles},
};

/// WorldState::new_blank(CreateBlankWorld) -> WorldState
pub struct CreateBlankWorld {
    /// All the data that can be used in the world
    pub data_handles: DestructedDataHandles,
    /// All the gates that can be used in the world
    pub gate_handles: DestructedGateHandles,
}

impl CreateBlankWorld {
    pub fn empty() -> Self {
        Self {
            data_handles: HashMap::new(),
            gate_handles: HashMap::new(),
        }
    }
}

/// WorldState::create_default_gate(CreateDefaultGate) -> Result&lt;ComponentId&gt;
pub struct CreateDefaultGate {
    /// Identifier of the gate type
    pub gate: ComponentVersion,
}

pub struct RegisterNewGateOutputByIndex {
    pub gate_output_socket: GateOutputSocket,
}
