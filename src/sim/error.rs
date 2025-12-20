use crate::common::world::{ComponentId, ComponentLibMinorId};

pub enum Error {
    /// Missing data in world
    MissingData { component_ids: Vec<ComponentId> },
    /// Missing data type in world
    MissingDataType { data_ident: ComponentLibMinorId },
    /// Missing data type in world, cannot call drop_mem and is leaking memory
    MissingDataTypeLeak { data_ident: ComponentLibMinorId },
}
