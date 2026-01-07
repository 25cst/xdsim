use crate::common::world::{ComponentId, ComponentVersion};

pub enum Error {
    /// Missing data in world
    MissingData { component_ids: Vec<ComponentId> },
    /// Missing data type in world
    MissingDataType { requested_type: String },
    /// Single error emitted by tick_all
    /// as of now, tick_all only emits
    /// - MissingData
    TickallErrors { errors: Vec<TickAllErrorEntry> },
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
