use std::{collections::HashMap, rc::Rc};

use crate::{
    common::world::{ComponentId, ComponentLibMinorId},
    packages::destructor::{DestructedData, DestructedGate},
    sim::component::{SimData, SimGate},
};

/// The representation for simulation state
pub struct WorldState {
    data: WorldStateData,
    gates: WorldStateGates,
}

pub struct WorldStateData {
    /// all data types
    handles: HashMap<ComponentLibMinorId, Rc<DestructedData>>,

    /// all buffers that have content
    /// the componentID is the connections holding the data
    readonly: HashMap<ComponentId, SimData>,
    writeonly: HashMap<ComponentId, SimData>,
}

pub struct WorldStateGates {
    /// all gate types
    handles: HashMap<ComponentLibMinorId, Rc<DestructedGate>>,

    /// all gates in world
    gates: HashMap<ComponentId, SimGate>,
}
