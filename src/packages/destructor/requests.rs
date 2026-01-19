use crate::{common::world::ComponentVersion, packages::loader::LibraryHandle};

pub struct DestructRequest {
    library: LibraryHandle,
    component_id: ComponentVersion,
}

impl DestructRequest {
    pub fn new(library: LibraryHandle, component_id: ComponentVersion) -> Self {
        Self {
            library,
            component_id,
        }
    }

    pub fn get_library(&self) -> &LibraryHandle {
        &self.library
    }

    pub fn get_component_id(&self) -> &ComponentVersion {
        &self.component_id
    }

    pub fn into_library(self) -> LibraryHandle {
        self.library
    }
}
