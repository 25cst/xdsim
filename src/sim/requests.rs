use std::collections::HashMap;

use crate::sim::sim_world::{DestructedDataHandles, DestructedGateHandles};

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
