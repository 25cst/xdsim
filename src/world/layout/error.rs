use crate::{common::world::ComponentId, world::sim};

pub enum Error {
    /// error originating from the simulation state
    Sim { reason: Box<sim::Error> },
    /// point in a connection not found
    ConnPointNotFound { point: ComponentId },
    /// segment in a connection not found
    ConnSegmentNotFound { segment: ComponentId },
    /// trying to bind to a point that is already binded to an producer
    /// or has an incoming segment
    ConnPointBindNonDanglingToProducer { point: ComponentId },
}

impl Error {
    pub fn from_sim(sim_error: Box<sim::Error>) -> Self {
        Self::Sim { reason: sim_error }
    }
}
