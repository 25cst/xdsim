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
    NewSegmentDirectionConflict {
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
    /// operating on a segment that needs to be dangling
    /// but it is not
    SegmentNotDangling {
        segment_id: ComponentId,
    },
    /// supplied length is out of expected value bounds
    LengthOutOfBounds {
        min: f64,
        max: Option<f64>,
        got: f64,
    },
}

impl Error {
    pub fn from_sim(sim_error: Box<sim::Error>) -> Self {
        Self::Sim { reason: sim_error }
    }
}
