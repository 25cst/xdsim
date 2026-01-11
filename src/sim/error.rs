use crate::common::world::{ComponentId, ComponentVersion, ComponentVersionReq};

pub enum Error {
    /// Missing data in world
    MissingData { component_ids: Vec<ComponentId> },
    /// Missing data type in world (requested with semver)
    MissingDataType { data_type: ComponentVersion },
    /// Missing data type in world (requested with a semver pattern)
    MissingRequestedDataType { data_type: ComponentVersionReq },
    /// Missing gate type in world (requested with semver)
    MissingGateType { gate_type: ComponentVersion },
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
        gate_id: ComponentId,
        output_list_length: usize,
        requested_index: usize,
    },
    /// Registering an output for a gate when it is already registered to an output
    GateOutputDoubleRegister {
        gate_type: ComponentVersion,
        gate_id: ComponentId,
        requested_index: usize,
    },
    /// Unregisters an output, but it is not registered in the first place
    GateOutputUnregisterNothing {
        gate_type: ComponentVersion,
        gate_id: ComponentId,
        requested_index: usize,
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
