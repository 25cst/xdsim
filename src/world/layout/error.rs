use crate::{
    common::world::{ComponentId, GateConsumerSocket, GateProducerSocket},
    world::sim,
};

pub enum Error {
    /// error originating from the simulation state
    Sim { reason: Box<sim::Error> },
    /// point in a connection not found
    ConnPointNotFound { point: ComponentId },
    /// segment in a connection not found
    ConnSegmentNotFound { segment: ComponentId },
    /// binding two producers to a conn
    ConnPointDoubleBindProducer { point: ComponentId },
    /// binding two conn points to a consumer
    DoubleBindConsumer { point: ComponentId },
    /// conn not exist in world when requested
    ConnNotFound { conn: ComponentId },
    /// No gate with requested ID in layout world
    GateNotFound { gate_id: ComponentId },
    /// no such consumer socket
    ConsumerSocketNotFound { socket: GateConsumerSocket },
    /// no such producer socket
    ProducerSocketNotFound { socket: GateProducerSocket },
}

impl Error {
    pub fn from_sim(sim_error: Box<sim::Error>) -> Self {
        Self::Sim { reason: sim_error }
    }
}
