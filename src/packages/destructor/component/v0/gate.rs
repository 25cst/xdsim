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
            self, DestructRequest, DestructedGateConsumerEntry, DestructedGateDefinition,
            DestructedGateProducerEntry,
        },
    },
};

pub struct DestructedGate {
    pub tick: extern "C" fn(GateMut, *const Slice) -> Slice,
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
        let consumers: &[GateConsumerEntry] = slice::from_slice(&definition.consumers);
        let producers: &[GateProducerEntry] = slice::from_slice(&definition.producers);

        pub fn to_consumer_entries(
            entries: &[GateConsumerEntry],
            gate_id: &ComponentVersion,
        ) -> Result<Vec<DestructedGateConsumerEntry>, destructor::Error> {
            let mut out = Vec::with_capacity(entries.len());

            for entry in entries {
                let GateConsumerEntry {
                    name,
                    data_type_req:
                        ComponentIdent {
                            package,
                            version,
                            component,
                        },
                    position,
                } = entry;

                out.push(DestructedGateConsumerEntry {
                    name: slice::from_str(name),
                    data_type_req: ComponentVersionReq {
                        package: slice::from_str(package),
                        component: slice::from_str(component),
                        version_req: VersionReq::parse(&slice::from_str(version)).map_err(|e| {
                            destructor::Error::InvalidVersionReq {
                                component: Box::new(gate_id.clone()),
                                version: slice::from_str(version),
                                reason: e.to_string(),
                            }
                        })?,
                    },
                    position: (*position).into(),
                })
            }

            Ok(out)
        }

        pub fn to_producer_entries(
            entries: &[GateProducerEntry],
            gate_id: &ComponentVersion,
        ) -> Result<Vec<DestructedGateProducerEntry>, destructor::Error> {
            let mut out = Vec::with_capacity(entries.len());

            for entry in entries {
                let GateProducerEntry {
                    name,
                    data_type:
                        ComponentIdent {
                            package,
                            version,
                            component,
                        },
                    position,
                } = entry;

                out.push(DestructedGateProducerEntry {
                    name: slice::from_str(name),
                    data_type: ComponentVersion {
                        package: slice::from_str(package),
                        component: slice::from_str(component),
                        version: Version::parse(&slice::from_str(version)).map_err(|e| {
                            destructor::Error::InvalidVersionReq {
                                component: Box::new(gate_id.clone()),
                                version: slice::from_str(version),
                                reason: e.to_string(),
                            }
                        })?,
                    },
                    position: *position,
                })
            }

            Ok(out)
        }

        Ok(DestructedGateDefinition {
            consumers: to_consumer_entries(consumers, gate_id)?,
            producers: to_producer_entries(producers, gate_id)?,
            bounding_box: definition.bounding_box,
        })
    }
}
