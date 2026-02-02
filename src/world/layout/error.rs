use crate::{
    common::world::{ComponentId, Direction},
    world::sim,
};

pub enum Error {
    /// error originating from the simulation state
    Sim {
        reason: Box<sim::Error>,
    },
    /// issues to do with new segment direction, such as:
    /// - new segment same direction as previous segment, which is not allowed
    /// - two segments have the same direction and origin, which is not allowed
    NewSegmentSameDirection {
        segment_id: ComponentId,
        direction: Direction,
    },
    NewSegmentOnSocket {
        segment_id: ComponentId,
    },
    /// segment not exist
    SegmentNotFound {
        segment_id: ComponentId,
    },
}

impl Error {
    pub fn from_sim(sim_error: Box<sim::Error>) -> Self {
        Self::Sim { reason: sim_error }
    }
}
