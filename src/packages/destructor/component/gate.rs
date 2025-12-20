use xdsim_cbinds::{
    common::{Direction, Slice, Vec2},
    v0::{
        app_state::PropertiesMut,
        component::{Gate, GateDefinition, GateMut},
        graphics::Graphic,
    },
};

use crate::{
    common::world::{ComponentLibMinorId, ComponentLibPatchId, GatePtr, GatePtrMut},
    packages::{
        destructor::{self, DestructRequest, component::v0},
        loader::LibraryHandle,
    },
};

/// Destructs a library into gate functions
///
/// Note: a copy of library is held for the functions to remain valid
pub struct DestructedGate {
    _library: LibraryHandle,
    handle: DestructedGateHandle,
}

/// version-generic gate definition
pub struct DestructedGateDefinition {
    /// inputs in the order they should appear in the slice
    pub inputs: Vec<DestructedGateIOEntry>,
    /// outputs in the order they should appear in the slice
    pub outputs: Vec<DestructedGateIOEntry>,
    /// The visual bounding box (dimension) of the gate
    /// The bottom left corner is (0, 0), top right corner is (width, height)
    pub bounding_box: Vec2,
}

pub struct DestructedGateIOEntry {
    pub name: String,
    pub data_type: ComponentLibPatchId,
    pub position: Vec2,
}

pub enum DestructedGateHandle {
    V0(v0::DestructedGate),
}

impl DestructedGate {
    pub fn new(request: DestructRequest) -> Result<Self, destructor::Error> {
        let get_schema_version: fn() -> u32 = *request
            .get_library()
            .get_symbol("schema_version", request.get_path())
            .map_err(destructor::Error::from_get_symbol)?;

        let handle = match get_schema_version() {
            0 => DestructedGateHandle::V0(v0::DestructedGate::new(&request)?),
            unsupported_version => {
                return Err(destructor::Error::UnsupportedSchemaVersion {
                    version: unsupported_version,
                });
            }
        };

        Ok(Self {
            _library: request.into_library(),
            handle,
        })
    }

    /// the slice is an array of *mut Data
    pub fn tick(&self, gate: GatePtrMut, inputs: Slice) -> Slice {
        match &self.handle {
            DestructedGateHandle::V0(handle) => (handle.tick)(gate, inputs),
        }
    }

    pub fn draw(&self, gate: GatePtr, direction: Direction, bounding_box: Vec2) -> Graphic {
        match &self.handle {
            DestructedGateHandle::V0(handle) => (handle.draw)(gate, direction, bounding_box),
        }
    }

    /*
    pub fn definition(&self, gate: GatePtr) -> GateDefinition {
        match &self.handle {
            DestructedGateHandle::V0(handle) => (handle.definition)(gate),
        }
    }
    */

    pub fn normalised_definition(&self, gate: GatePtr) -> DestructedGateDefinition {
        match &self.handle {
            DestructedGateHandle::V0(handle) => handle.get_normalised_definition(gate),
        }
    }

    pub fn properties(&self, gate: GatePtrMut) -> PropertiesMut {
        match &self.handle {
            DestructedGateHandle::V0(handle) => (handle.properties)(gate),
        }
    }

    /// this is guaranteed to succeed
    /// (unless the component file throws an error)
    pub fn serialize(&self, gate: GatePtr) -> Slice {
        match &self.handle {
            DestructedGateHandle::V0(handle) => (handle.serialize)(gate),
        }
    }

    /// if deserialize fails, returns a None
    /// DataMut is guaranteed to be not null
    /// (unless the component file throws an error)
    pub fn deserialize(&self, bytes: &Slice) -> Option<GatePtrMut> {
        match &self.handle {
            DestructedGateHandle::V0(handle) => {
                let ptr = (handle.deserialize)(bytes);
                if ptr.is_null() { None } else { Some(ptr) }
            }
        }
    }

    /// this is guaranteed to succeed
    /// (unless the component file throws an error)
    pub fn default_value(&self) -> GatePtrMut {
        match &self.handle {
            DestructedGateHandle::V0(handle) => (handle.default_value)(),
        }
    }

    /// this is guaranteed to succeed
    /// it is important for the pointer to be valid
    /// otherwise this will lead to a double free or segfault
    /// (unless the component file throws an error, or the data is already been dropped)
    pub fn drop_mem(&self, gate: GatePtrMut) {
        match &self.handle {
            DestructedGateHandle::V0(handle) => (handle.drop_mem)(gate),
        }
    }
}
