use std::collections::HashMap;

use crate::{common::world::ComponentId, world::layout::component::LayoutGate};

/// layout world state gate shadows over the sim world state
///
/// gate IDs satisfies constraint
pub struct WorldStateGates {
    gates: HashMap<ComponentId, LayoutGate>,
}
