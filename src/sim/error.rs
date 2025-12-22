use crate::common::world::{ComponentId, ComponentLibMinorId};

pub enum Error {
    /// Missing data in world
    MissingData { component_ids: Vec<ComponentId> },
    /// Missing data type in world
    MissingDataType { data_ident: ComponentLibMinorId },
    /// Missing data type in world, cannot call drop_mem and is leaking memory
    MissingDataTypeLeak { data_ident: ComponentLibMinorId },
    /// Single error emitted by tick_all
    /// as of now, tick_all only emits
    /// - MissingData
    /// - MissingDataType
    /// - MissingDataTypeLeak
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
