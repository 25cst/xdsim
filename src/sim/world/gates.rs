use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    common::world::ComponentId,
    packages::destructor::DestructedGate,
    sim::{self, component::SimGate, error::TickAllErrorEntry, world::*},
};

pub type DestructedGateHandles =
    HashMap<PackageName, HashMap<PackageVersion, HashMap<ComponentName, Rc<DestructedGate>>>>;

pub struct WorldStateGates {
    /// all gate types
    handles: DestructedGateHandles,

    /// all gates in world
    gates: HashMap<ComponentId, SimGate>,

    /// For a buffer, which gates are using that buffer as input?
    /// Map<Buffer ID, Gate IDs>
    ///
    /// Constraints:
    /// - The gate IDs must exist in Self.gates
    gates_with_input: HashMap<ComponentId, HashSet<ComponentId>>,

    /// For a buffer, what is the gate outputing to the buffer?
    /// Map<Buffer ID, Gate ID>
    ///
    /// Constraints:
    /// - The gate IDs must exist in Self.gates
    gate_with_output: HashMap<ComponentId, ComponentId>,
}

impl WorldStateGates {
    /// create world state gates with only handles and no gates in world
    pub fn new_blank(handles: DestructedGateHandles) -> Self {
        Self {
            handles,
            gates: HashMap::new(),
            gates_with_input: HashMap::new(),
            gate_with_output: HashMap::new(),
        }
    }

    // strictly speaking the compiler doesnt require this to SimGate::tick to be mut
    // but I've marked it as so because it would make sense
    // if it is causing trouble, we can remove it
    pub fn tick_all(&mut self, world_data: &mut WorldStateData) -> Result<(), sim::Error> {
        let mut tick_errors = Vec::new();

        for (gate_id, gate) in self.gates.iter_mut() {
            if let Err(e) = gate.tick(world_data) {
                tick_errors.push(TickAllErrorEntry::new(*gate_id, e));
            }
        }

        if tick_errors.is_empty() {
            Ok(())
        } else {
            Err(sim::Error::TickallErrors {
                errors: tick_errors,
            })
        }
    }
}
