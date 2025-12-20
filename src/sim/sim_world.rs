use std::{collections::HashMap, rc::Rc};

use crate::{
    common::world::{ComponentId, ComponentLibMinorId},
    packages::destructor::DestructedData,
    sim::component::{SimData, SimGate},
};

/// The representation for simulation state
pub struct WorldState {}

pub struct WorldStateData {
    /// all
    data_handles: HashMap<ComponentLibMinorId, Rc<DestructedData>>,
    // the componentID is the connections holding the data
    data_readonly: HashMap<ComponentId, SimData>,
    data_writeonly: HashMap<ComponentId, SimData>,
}

pub struct WorldStateGates {
    gates: HashMap<ComponentId, SimGate>,
}
