use crate::common::world::{ComponentId, ComponentLibMinorId};

pub enum Error {
    /// Missing data in world
    MissingData { component_ids: Vec<ComponentId> },
    /// Missing data type in world
    MissingDataType { data_ident: ComponentLibMinorId },
    /// Single error emitted by tick_all
    /// as of now, tick_all only emits
    /// - MissingData
    TickallErrors { errors: Vec<TickAllErrorEntry> },
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
