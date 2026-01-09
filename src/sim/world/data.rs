use std::{collections::HashMap, mem, rc::Rc};

use crate::{
    common::world::{ComponentId, ComponentVersion, ComponentVersionReq},
    packages::destructor::DestructedData,
    sim::{component::SimData, world::*},
};

pub type DestructedDataHandles =
    HashMap<PackageName, HashMap<PackageVersion, HashMap<ComponentName, Rc<DestructedData>>>>;

pub struct WorldStateData {
    /// all data types
    // may one day replace the Rc in SimData with a dumb pointer because it is guaranteed to exist
    // as owned here
    handles: DestructedDataHandles,

    /// all buffers that have content
    /// the componentID is the connections holding the data
    readonly: HashMap<ComponentId, SimData>,
    writeonly: HashMap<ComponentId, SimData>,
}

impl WorldStateData {
    /// create world state data with only handles and no buffers in world
    pub fn new_blank(handles: DestructedDataHandles) -> Self {
        Self {
            handles,
            readonly: HashMap::new(),
            writeonly: HashMap::new(),
        }
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
            .find(|(version, _)| component_req.version_req.matches(version))?
            .1
            .get(&component_req.component)
    }

    /// read from current world state
    pub fn read_buffer(&self, buf_id: &ComponentId) -> Option<&SimData> {
        self.readonly.get(buf_id)
    }

    /// write to next tick's world state
    pub fn write_buffer(&mut self, buf_id: ComponentId, content: SimData) {
        self.writeonly.insert(buf_id, content);
    }

    /// end the current tick and write all updates in the current tick to world state
    pub fn flush(&mut self) {
        mem::swap(&mut self.readonly, &mut self.writeonly); // i'm so cool
        self.writeonly.clear();
    }
}
