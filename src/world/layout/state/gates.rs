use std::collections::HashMap;

use crate::{
    common::world::{ComponentId, Vec2},
    world::{layout::component::LayoutGate, sim::SimGate},
};

/// layout world state gate shadows over the sim world state
///
/// gate IDs satisfies constraints
/// - the key set of layout::WorldGates and sim::WorldGates are the same,
///   aka a gate exist in sim world state iff it exist in layout state
pub struct WorldStateGates {
    gates: HashMap<ComponentId, LayoutGate>,
}

impl WorldStateGates {
    /// create new layout gate state
    pub fn new_blank() -> Self {
        Self {
            gates: HashMap::new(),
        }
    }

    /// DANGER: unchecked operation, does not guarantee that the gate exist in sim world,
    /// also does not guarantee this gate is not already in hashmap
    ///
    /// adds new layout gate into layout,
    pub fn add_gate(&mut self, gate_id: ComponentId, origin: Vec2, sim_gate: &SimGate) {
        self.gates
            .insert(gate_id, LayoutGate::new(origin, sim_gate));
    }
}
