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
    /// An input is bound to an output socket, but that output socket does not exist
    OutputSocketNotFound { output_socket: GateOutputSocket },
    /// an input socket is bound to the same output 2 times
    OutputSocketDoubleBound {
        input_socket: GateInputSocket,
        output_socket: GateOutputSocket,
    },
    /// An input is bound to an output socket, but that input socket does not exist
    InputSocketNotFound { input_socket: GateInputSocket },
    /// An input is already bound to an output, but it is requested to bound to another output
    InputSocketDoubleBound {
        input_socket: GateInputSocket,
        current_output_source: GateOutputSocket,
        new_output_source: GateOutputSocket,
    },
    /// An input socket is connected to an output socket but their data_types do not match
    IOTypeMismatch {
        input_socket: GateInputSocket,
        output_socket: GateOutputSocket,
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
