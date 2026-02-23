use crate::common::world::ComponentId;

pub enum Error {
    /// attempting to unregister a nonexisting component id
    UnregisterNonexistentComponentId { id: ComponentId },
}
