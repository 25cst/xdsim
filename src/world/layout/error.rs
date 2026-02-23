use crate::{
    common::{
        self,
        world::{ComponentId, GateConsumerSocket, GateProducerSocket},
    },
    world::{layout::SegmentDraw, sim},
};

pub enum Error {
    /// error originating from the simulation state
    Sim(Box<sim::Error>),
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
    GateNotFound { gate: ComponentId },
    /// no such consumer socket
    ConsumerSocketNotFound { socket: GateConsumerSocket },
    /// no such producer socket
    ProducerSocketNotFound { socket: GateProducerSocket },
    /// removing a point that still have stuff connected to it
    RmNonEmptyPoint { point: ComponentId },
    /// removing a segment that still have stuff connected to it
    RmNonEmptySegment { segment: ComponentId },
    /// crate common error
    Common(Box<common::Error>),
    /// unsupported segment draw operation
    SegmentDrawUnsupported { request: SegmentDraw },
}
