use std::{collections::HashMap, mem, rc::Rc};

use crate::{
    common::world::{ComponentId, ComponentLibMinorId},
    packages::destructor::{DestructedData, DestructedGate},
    sim::{
        self,
        component::{SimData, SimGate},
        error::TickAllErrorEntry,
    },
};

/// The representation for simulation state
pub struct WorldState {
    data: WorldStateData,
    gates: WorldStateGates,
}

impl WorldState {
    /// tick the current world
    /// if this function returns error, its not end of the world
    /// it just means a buffer is used as input to a gate, but is not present
    /// this could be caused by bad implementation for edge cases such as:
    /// - new connection just added
    /// - an existing connection just been removed
    /// for a good implementation this should not happen
    /// if an error is given, simply put it in debug logs or somewhere else
    pub fn tick_all(&mut self) -> Result<(), sim::Error> {
        let res = self.gates.tick_all(&mut self.data);
        self.data.flush();
        res
    }
}

pub struct WorldStateData {
    /// all data types
    // may one day replace the Rc in SimData with a dumb pointer because it is guaranteed to exist
    // as owned here
    handles: HashMap<ComponentLibMinorId, Rc<DestructedData>>,

    /// all buffers that have content
    /// the componentID is the connections holding the data
    readonly: HashMap<ComponentId, SimData>,
    writeonly: HashMap<ComponentId, SimData>,
}

impl WorldStateData {
    pub fn get_handle(&self, data_lib_ident: &ComponentLibMinorId) -> Option<&Rc<DestructedData>> {
        self.handles.get(data_lib_ident)
    }

    /// read from current world state
    pub fn read_buffer(&self, buf_id: &ComponentId) -> Option<&SimData> {
        self.readonly.get(buf_id)
    }

    /// write to next tick's world state
    pub fn write_buffer(&mut self, buf_id: ComponentId, content: SimData) {
        self.writeonly.insert(buf_id, content);
    }

    /// end the current tick and write all updates in the current tick to world state
    pub fn flush(&mut self) {
        mem::swap(&mut self.readonly, &mut self.writeonly); // i'm so cool
        self.writeonly.clear();
    }
}

pub struct WorldStateGates {
    /// all gate types
    handles: HashMap<ComponentLibMinorId, Rc<DestructedGate>>,

    /// all gates in world
    gates: HashMap<ComponentId, SimGate>,
}

impl WorldStateGates {
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
