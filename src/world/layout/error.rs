use crate::{common::world::ComponentId, world::sim};

pub enum Error {
    /// error originating from the simulation state
    Sim {
        reason: Box<sim::Error>,
    },
    ConnPointNotFound {
        point: ComponentId,
    },
    ConnSegmentNotFound {
        segment: ComponentId,
    },
}

impl Error {
    pub fn from_sim(sim_error: Box<sim::Error>) -> Self {
        Self::Sim { reason: sim_error }
    }
}
