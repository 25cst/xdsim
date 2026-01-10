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
