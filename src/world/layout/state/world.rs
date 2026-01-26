use crate::world::{layout::state::conns::WorldStateConns, sim};

pub struct WorldState {
    /// wrapped simulation world
    sim_state: sim::WorldState,

    /// positions and paths of conns
    conns: WorldStateConns,
}
