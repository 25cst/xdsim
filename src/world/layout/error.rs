use crate::{common::world::ComponentId, world::sim};

pub enum Error {
    /// error originating from the simulation state
    Sim { reason: Box<sim::Error> },
    /// point in a connection not found
    ConnPointNotFound { point: ComponentId },
    /// segment in a connection not found
    ConnSegmentNotFound { segment: ComponentId },
    /// binding two producers to a conn
    ConnPointDoubleBindProducer { point: ComponentId },
}

impl Error {
    pub fn from_sim(sim_error: Box<sim::Error>) -> Self {
        Self::Sim { reason: sim_error }
    }
}
