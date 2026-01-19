use std::rc::Rc;

use crate::{
    common::world::{ComponentVersion, ComponentVersionReq},
    packages::destructor::DestructedData,
    sim::requests::DestructedDataHandles,
};

pub struct WorldStateData {
    /// all data types
    // may one day replace the Rc in SimData with a dumb pointer because it is guaranteed to exist
    // as owned here
    handles: DestructedDataHandles,
}

impl WorldStateData {
    /// create world state data with only handles and no buffers in world
    pub fn new_blank(handles: DestructedDataHandles) -> Self {
        Self { handles }
    }

    /// Get handle using a ComponentVersion
    pub fn get_handle(&self, component: &ComponentVersion) -> Option<&Rc<DestructedData>> {
        self.handles
            .get(&component.package)?
            .get(&component.version)?
            .get(&component.component)
    }

    /// Get handle using a ComponentVersionReq
    pub fn request_handle(
        &self,
        component_req: &ComponentVersionReq,
    ) -> Option<&Rc<DestructedData>> {
        self.handles
            .get(&component_req.package)?
            .iter()
            .rfind(|(version, _)| component_req.version_req.matches(version))?
            .1
            .get(&component_req.component)
    }
}
