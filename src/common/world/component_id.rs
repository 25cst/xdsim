//! ID of a component in both the simulation and graphics world
#[derive(Hash, PartialEq, Eq)]
pub struct ComponentId(u32);

impl ComponentId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}
