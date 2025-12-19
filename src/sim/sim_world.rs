//! The representation for simulation state
use std::collections::HashMap;

use crate::{common::world::ComponentId, sim::component::SimData};

pub struct WorldState {
    data_fixed: HashMap<ComponentId, SimData>,
    data_writable: HashMap<ComponentId, SimData>,
}
