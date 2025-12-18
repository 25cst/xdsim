use xdsim_cbinds::v0::{app_state::PropertiesMut, common::Slice, component::*, graphics::Graphic};

use crate::packages::destructor::{self, DestructRequest};

pub struct DestructedGate {
    tick: fn(GateMut, *const GateTickRequest) -> Slice,
    draw: fn(Gate, *const GateDrawRequest) -> Graphic,
    definition: fn(Gate) -> GateDefinition,
    properties: fn(GateMut) -> PropertiesMut,
    serialize: fn(Gate) -> Slice,
    deserialize: fn(Slice) -> GateMut,
    default_value: fn() -> GateMut,
    drop_mem: fn(GateMut),
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
}
