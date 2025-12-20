use xdsim_cbinds::{
    common::*,
    v0::{app_state::PropertiesMut, component::*, graphics::Graphic},
};

use crate::{
    common::world::{ComponentLibPatchId, GatePtr},
    packages::{
        chelper::slice,
        destructor::{self, DestructRequest, DestructedGateDefinition, DestructedGateIOEntry},
    },
};

pub struct DestructedGate {
    pub tick: fn(GateMut, Slice) -> Slice,
    pub draw: fn(Gate, Direction, Vec2) -> Graphic,
    pub definition: fn(Gate) -> GateDefinition,
    pub properties: fn(GateMut) -> PropertiesMut,
    pub serialize: fn(Gate) -> Slice,
    pub deserialize: fn(*const Slice) -> GateMut,
    pub default_value: fn() -> GateMut,
    pub drop_mem: fn(GateMut),
}

impl DestructedGate {
    pub fn new(request: &DestructRequest) -> Result<Self, destructor::Error> {
        Ok(Self {
            tick: *request
                .get_library()
                .get_symbol("gate_tick", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            draw: *request
                .get_library()
                .get_symbol("gate_draw", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            definition: *request
                .get_library()
                .get_symbol("gate_def", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            properties: *request
                .get_library()
                .get_symbol("gate_props", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            serialize: *request
                .get_library()
                .get_symbol("gate_serialize", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            deserialize: *request
                .get_library()
                .get_symbol("gate_deserialize", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            default_value: *request
                .get_library()
                .get_symbol("gate_default", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            drop_mem: *request
                .get_library()
                .get_symbol("gate_drop", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
        })
    }

    pub fn get_normalised_definition(&self, gate: GatePtr) -> DestructedGateDefinition {
        let definition = (self.definition)(gate);
        let inputs: &[GateIOEntry] = slice::from_slice(&definition.inputs);
        let outputs: &[GateIOEntry] = slice::from_slice(&definition.outputs);

        pub fn map_io_entries(entries: &[GateIOEntry]) -> Vec<DestructedGateIOEntry> {
            entries
                .iter()
                .map(
                    |GateIOEntry {
                         name,
                         data_type:
                             ComponentIdent {
                                 package,
                                 component,
                                 major,
                                 minor,
                                 patch,
                             },
                         position,
                     }| DestructedGateIOEntry {
                        name: slice::from_str(name),
                        data_type: ComponentLibPatchId {
                            package: slice::from_str(package),
                            component: slice::from_str(component),
                            major: *major,
                            minor: *minor,
                            patch: *patch,
                        },
                        position: *position,
                    },
                )
                .collect()
        }

        DestructedGateDefinition {
            inputs: map_io_entries(inputs),
            outputs: map_io_entries(outputs),
            bounding_box: definition.bounding_box,
        }
    }
}
