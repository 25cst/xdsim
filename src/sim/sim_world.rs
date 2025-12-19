use std::collections::HashMap;

use crate::{common::world::ComponentId, sim::component::SimData};

/// The representation for simulation state
pub struct WorldState {
    data_fixed: HashMap<ComponentId, SimData>,
    data_writable: HashMap<ComponentId, SimData>,
}
