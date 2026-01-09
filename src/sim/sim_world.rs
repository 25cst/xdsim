use std::{collections::HashMap, mem, rc::Rc};

use semver::Version;

use crate::{
    common::world::{ComponentId, ComponentVersion, ComponentVersionReq},
    packages::destructor::{DestructedData, DestructedGate},
    sim::{
        self,
        component::{SimData, SimGate},
        error::TickAllErrorEntry,
        requests::CreateBlankWorld,
    },
};

/// The representation for simulation state
pub struct WorldState {
    data: WorldStateData,
    gates: WorldStateGates,
}

impl WorldState {
    /// create new empty world with library handles to gates and data
    pub fn new_blank(request: CreateBlankWorld) -> Self {
        Self {
            data: WorldStateData::new_blank(request.data_handles),
            gates: WorldStateGates::new_blank(request.gate_handles),
        }
    }

    /// tick the current world
    /// if this function returns error, its not end of the world
    /// it just means a buffer is used as input to a gate, but is not present
    /// this could be caused by bad implementation for edge cases such as:
    /// - new connection just added
    /// - an existing connection just been removed
    ///
    /// for a good implementation this should not happen.
    /// if an error is given, simply put it in debug logs or somewhere else
    pub fn tick_all(&mut self) -> Result<(), sim::Error> {
        let res = self.gates.tick_all(&mut self.data);
        self.data.flush();
        res
    }
}

pub type PackageName = String;
pub type PackageVersion = Version;
pub type ComponentName = String;

pub type DestructedDataHandles =
    HashMap<PackageName, HashMap<PackageVersion, HashMap<ComponentName, Rc<DestructedData>>>>;
pub type DestructedGateHandles =
    HashMap<PackageName, HashMap<PackageVersion, HashMap<ComponentName, Rc<DestructedGate>>>>;

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

pub struct WorldStateGates {
    /// all gate types
    handles: DestructedGateHandles,

    /// all gates in world
    gates: HashMap<ComponentId, SimGate>,
}

impl WorldStateGates {
    /// create world state gates with only handles and no gates in world
    pub fn new_blank(handles: DestructedGateHandles) -> Self {
        Self {
            handles,
            gates: HashMap::new(),
        }
    }

    // strictly speaking the compiler doesnt require this to SimGate::tick to be mut
    // but I've marked it as so because it would make sense
    // if it is causing trouble, we can remove it
    pub fn tick_all(&mut self, world_data: &mut WorldStateData) -> Result<(), sim::Error> {
        let mut tick_errors = Vec::new();

        for (gate_id, gate) in self.gates.iter_mut() {
            if let Err(e) = gate.tick(world_data) {
                tick_errors.push(TickAllErrorEntry::new(*gate_id, e));
            }
        }

        if tick_errors.is_empty() {
            Ok(())
        } else {
            Err(sim::Error::TickallErrors {
                errors: tick_errors,
            })
        }
    }
}
