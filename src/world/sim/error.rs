use crate::common::world::{
    ComponentId, ComponentVersion, ComponentVersionReq, GateConsumerSocket, GateProducerSocket,
};

#[derive(Debug)]
pub enum Error {
    TickSingleGate {
        gate_id: ComponentId,
        errors: Vec<Self>,
    },
    /// Missing data type in world (requested with semver)
    DataTypeNotFound { data_type: ComponentVersion },
    /// Missing data type in world (requested with a semver pattern)
    RequestedDataTypeNotFound { data_type: ComponentVersionReq },
    /// Missing gate type in world (requested with semver)
    GateTypeNotFound { gate_type: ComponentVersion },
    /// Single error emitted by tick_all
    /// as of now, tick_all only emits
    /// - MissingData
    TickallErrors { errors: Vec<TickAllErrorEntry> },
    /// Error parsing gate definition
    GateDefinition {
        component: ComponentVersion,
        reason: String,
    },
    /// Requests gate output by index but out of bounds
    GateProducerIndexOutOfBounds {
        gate_type: ComponentVersion,
        gate_socket: GateProducerSocket,
        producer_list_length: usize,
    },
    /// Requests gate input by index but out of bounds
    GateConsumerIndexOutOfBounds {
        gate_type: ComponentVersion,
        gate_socket: GateConsumerSocket,
        producer_list_length: usize,
    },
    /// Registering an input for a gate when it is already registered to an input
    GateConsumerDoubleRegister {
        gate_type: ComponentVersion,
        gate_socket: GateConsumerSocket,
    },
    /// Registering an output for a gate when it is already registered to an output
    GateProducerDoubleRegister {
        gate_type: ComponentVersion,
        gate_socket: GateProducerSocket,
    },
    /// Unregisters an output, but it is not registered in the first place
    GateProducerUnregisterNothing {
        gate_type: ComponentVersion,
        gate_socket: GateProducerSocket,
    },
    /// Unregisters an input, but it is not registered in the first place
    GateConsumerUnregisterNothing {
        gate_type: ComponentVersion,
        gate_socket: GateConsumerSocket,
    },
    /// No gate with requested ID in sim world
    GateNotFound { gate_id: ComponentId },
    /// An input is bound to an output socket, but that output socket does not exist
    ProducerSocketNotFound { producer_socket: GateProducerSocket },
    /// an input socket is bound to the same output 2 times
    ProducerSocketDoubleBound {
        consumer_socket: GateConsumerSocket,
        producer_socket: GateProducerSocket,
    },
    /// An input is bound to an output socket, but that input socket does not exist
    ConsumerSocketNotFound { consumer_socket: GateConsumerSocket },
    /// An input is already bound to an output, but it is requested to bound to another output
    ConsumerSocketDoubleBound {
        consumer_socket: GateConsumerSocket,
        current_producer: GateProducerSocket,
        new_producer: GateProducerSocket,
    },
    /// An input socket is connected to an output socket but their data_types do not match
    IOTypeMismatch {
        consumer_socket: GateConsumerSocket,
        producer_socket: GateProducerSocket,
    },
}

#[derive(Debug)]
pub struct TickAllErrorEntry {
    emitter: ComponentId,
    content: Error,
}

impl TickAllErrorEntry {
    pub fn get_emitter(&self) -> ComponentId {
        self.emitter
    }

    pub fn get_content(&self) -> &Error {
        &self.content
    }

    pub fn new(emitter: ComponentId, content: Error) -> Self {
        Self { emitter, content }
    }
}
