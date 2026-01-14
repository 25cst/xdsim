use semver::{Version, VersionReq};
use xdsim_cbinds::{
    common::*,
    v0::{app_state::PropertiesMut, component::*, graphics::Graphic},
};

use crate::{
    common::world::{ComponentVersion, ComponentVersionReq, GatePtr},
    packages::{
        chelper::slice,
        destructor::{
            self, DestructRequest, DestructedGateDefinition, DestructedGateInputEntry,
            DestructedGateOutputEntry,
        },
    },
};

pub struct DestructedGate {
    pub tick: extern "C" fn(GateMut, Slice) -> Slice,
    pub draw: extern "C" fn(Gate, Direction, Vec2) -> Graphic,
    pub definition: extern "C" fn(Gate) -> GateDefinition,
    pub properties: extern "C" fn(GateMut) -> PropertiesMut,
    pub serialize: extern "C" fn(Gate) -> Slice,
    pub deserialize: extern "C" fn(*const Slice) -> GateMut,
    pub default_value: extern "C" fn() -> GateMut,
    pub drop_mem: extern "C" fn(GateMut),
}

impl DestructedGate {
    pub fn new(request: &DestructRequest) -> Result<Self, destructor::Error> {
        Ok(Self {
            tick: *request
                .get_library()
                .get_symbol("gate_tick")
                .map_err(destructor::Error::from_get_symbol)?,
            draw: *request
                .get_library()
                .get_symbol("gate_draw")
                .map_err(destructor::Error::from_get_symbol)?,
            definition: *request
                .get_library()
                .get_symbol("gate_def")
                .map_err(destructor::Error::from_get_symbol)?,
            properties: *request
                .get_library()
                .get_symbol("gate_props")
                .map_err(destructor::Error::from_get_symbol)?,
            serialize: *request
                .get_library()
                .get_symbol("gate_serialize")
                .map_err(destructor::Error::from_get_symbol)?,
            deserialize: *request
                .get_library()
                .get_symbol("gate_deserialize")
                .map_err(destructor::Error::from_get_symbol)?,
            default_value: *request
                .get_library()
                .get_symbol("gate_default")
                .map_err(destructor::Error::from_get_symbol)?,
            drop_mem: *request
                .get_library()
                .get_symbol("gate_drop")
                .map_err(destructor::Error::from_get_symbol)?,
        })
    }

    pub fn get_normalised_definition(
        &self,
        gate: GatePtr,
        gate_id: &ComponentVersion,
    ) -> Result<DestructedGateDefinition, destructor::Error> {
        let definition = (self.definition)(gate);
        let inputs: &[GateInputEntry] = slice::from_slice(&definition.inputs);
        let outputs: &[GateOutputEntry] = slice::from_slice(&definition.outputs);

        pub fn to_input_entries(
            entries: &[GateInputEntry],
            gate_id: &ComponentVersion,
        ) -> Result<Vec<DestructedGateInputEntry>, destructor::Error> {
            let mut out = Vec::with_capacity(entries.len());

            for entry in entries {
                let GateInputEntry {
                    name,
                    data_type_req:
                        ComponentIdent {
                            package,
                            version,
                            component,
                        },
                    position,
                } = entry;

                out.push(DestructedGateInputEntry {
                    name: slice::from_str(name),
                    data_type_req: ComponentVersionReq {
                        package: slice::from_str(package),
                        component: slice::from_str(component),
                        version_req: match VersionReq::parse(&slice::from_str(version)) {
                            Ok(v) => v,
                            Err(e) => {
                                return Err(destructor::Error::InvalidVersionReq {
                                    component: Box::new(gate_id.clone()),
                                    version: slice::from_str(version),
                                    reason: e.to_string(),
                                });
                            }
                        },
                    },
                    position: *position,
                })
            }

            Ok(out)
        }

        pub fn to_output_entries(
            entries: &[GateOutputEntry],
            gate_id: &ComponentVersion,
        ) -> Result<Vec<DestructedGateOutputEntry>, destructor::Error> {
            let mut out = Vec::with_capacity(entries.len());

            for entry in entries {
                let GateOutputEntry {
                    name,
                    data_type:
                        ComponentIdent {
                            package,
                            version,
                            component,
                        },
                    position,
                } = entry;

                out.push(DestructedGateOutputEntry {
                    name: slice::from_str(name),
                    data_type: ComponentVersion {
                        package: slice::from_str(package),
                        component: slice::from_str(component),
                        version: match Version::parse(&slice::from_str(version)) {
                            Ok(v) => v,
                            Err(e) => {
                                return Err(destructor::Error::InvalidVersionReq {
                                    component: Box::new(gate_id.clone()),
                                    version: slice::from_str(version),
                                    reason: e.to_string(),
                                });
                            }
                        },
                    },
                    position: *position,
                })
            }

            Ok(out)
        }

        Ok(DestructedGateDefinition {
            inputs: to_input_entries(inputs, gate_id)?,
            outputs: to_output_entries(outputs, gate_id)?,
            bounding_box: definition.bounding_box,
        })
    }
}
