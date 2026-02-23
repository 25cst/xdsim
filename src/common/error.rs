use crate::common::world::{ComponentId, ComponentIdType, ComponentIdTypeName};

pub enum Error {
    /// attempting to unregister a nonexisting component id
    UnregisterNonexistentComponentId { id: ComponentId },
    /// looking up component type by ID but is not found
    ComponentIdLookupNotFound { id: ComponentId },
    ComponentIdTypeMismatch {
        id: ComponentId,
        expected: ComponentIdTypeName,
        got: ComponentIdType,
    },
}
