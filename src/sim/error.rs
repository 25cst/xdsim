use crate::common::world::{ComponentId, ComponentVersion, ComponentVersionReq, GateOutputSocket};

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
    /// Trying to register two producers for a buffer
    BufferDoubleProducerRegister {
        gate_type: ComponentVersion,
        gate_socket: GateOutputSocket,
        buffer_id: ComponentId,
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
    /// No gate with requested ID in world
    GateNotFound { gate_id: ComponentId },
    /// No buffer with requested ID in world
    BufferNotFound { buffer_id: ComponentId },
    /// A buffer is written to twice in a single tick, which is not allowed
    BufferDoubleWrite { buffer_id: ComponentId },
    /// Requested to remove buffer producer, but the buffer does not have a producer
    BufferNoProducerToRemove { buffer_id: ComponentId },
    BufferTypeMismatch {
        buffer_id: ComponentId,
        expected_type: ComponentVersion,
        got_type: ComponentVersion,
    },
}

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
