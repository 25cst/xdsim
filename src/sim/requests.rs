use std::collections::HashMap;

use crate::{
    common::world::ComponentVersion,
    sim::world::{DestructedDataHandles, DestructedGateHandles},
};

/// WorldState::new_blank(CreateBlankWorld)
pub struct CreateBlankWorld {
    pub data_handles: DestructedDataHandles,
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

pub struct CreateDefaultGate {
    pub gate: ComponentVersion,
}

impl CreateDefaultGate {
    pub fn new(gate: ComponentVersion) -> Self {
        Self { gate }
    }
}
