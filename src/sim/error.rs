use crate::common::world::{
    ComponentId, ComponentVersion, ComponentVersionReq, GateInputSocket, GateOutputSocket,
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
    GateOutputIndexOutOfBounds {
        gate_type: ComponentVersion,
        gate_socket: GateOutputSocket,
        output_list_length: usize,
    },
    /// Requests gate input by index but out of bounds
    GateInputIndexOutOfBounds {
        gate_type: ComponentVersion,
        gate_socket: GateInputSocket,
        output_list_length: usize,
    },
    /// Trying to register two producers for a buffer
    BufferDoubleProducerRegister {
        gate_type: ComponentVersion,
        gate_socket: GateOutputSocket,
        buffer_id: ComponentId,
    },
    /// Registering an input for a gate when it is already registered to an input
    GateInputDoubleRegister {
        gate_type: ComponentVersion,
        gate_socket: GateInputSocket,
    },
    /// Registering an output for a gate when it is already registered to an output
    GateOutputDoubleRegister {
        gate_type: ComponentVersion,
        gate_socket: GateOutputSocket,
    },
    /// Unregisters an output, but it is not registered in the first place
    GateOutputUnregisterNothing {
        gate_type: ComponentVersion,
        gate_socket: GateOutputSocket,
    },
    /// Unregisters an input, but it is not registered in the first place
    GateInputUnregisterNothing {
        gate_type: ComponentVersion,
        gate_socket: GateInputSocket,
    },
    /// No gate with requested ID in world
    GateNotFound { gate_id: ComponentId },
    /// No buffer with requested ID in world
    BufferNotFound { buffer_id: ComponentId },
    /// A buffer is written to twice in a single tick, which is not allowed
    BufferDoubleWrite { buffer_id: ComponentId },
    /// Requested to remove buffer producer, but the buffer does not have a producer
    BufferNoProducerToRemove { buffer_id: ComponentId },
    /// basic type error when socket and buffer type mismatches
    BufferTypeMismatch {
        buffer_id: ComponentId,
        /// buffer type
        expected_type: ComponentVersion,
        /// producer socket type
        got_type: ComponentVersion,
    },
    /// basic type error when socket (request) and buffer type (concrete) mismatches
    BufferTypeReqMismatch {
        buffer_id: ComponentId,
        /// consumer socket type
        expected_type: ComponentVersionReq,
        /// buffer type
        got_type: ComponentVersion,
    },
    /// trying to remove a nonexistend consumer
    ///
    /// this should never happen as the case would've already been covered by SimGate, but I
    /// decided to also make WorldData satisfy constraints if SimGate goes wrong
    BufferConsumerToRemoveNonexistent {
        buffer_id: ComponentId,
        consumer_socket: GateInputSocket,
    },
    /// trying to add an already existing consumer
    ///
    /// this should never happen as the case would've already been covered by SimGate, but I
    /// decided to also make WorldData satisfy constraints if SimGate goes wrong
    BufferDoubleConsumerRegister {
        buffer_id: ComponentId,
        consumer_socket: GateInputSocket,
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
