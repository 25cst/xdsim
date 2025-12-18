use xdsim_cbinds::v0::{
    app_state::PropertiesMut,
    common::Slice,
    component::{Connection, ConnectionDefinition, ConnectionDrawRequest, ConnectionMut},
    graphics::Graphic,
};

use crate::packages::destructor::{self, DestructRequest};

pub struct DestructedConnection {
    draw: fn(Connection, *const ConnectionDrawRequest) -> Graphic,
    definition: fn(Connection) -> ConnectionDefinition,
    properties: fn(ConnectionMut) -> PropertiesMut,
    serialize: fn(Connection) -> Slice,
    deserialize: fn(Slice) -> ConnectionMut,
    default_value: fn() -> ConnectionMut,
    drop_mem: fn(ConnectionMut),
}

impl DestructedConnection {
    pub fn new(request: &DestructRequest) -> Result<Self, destructor::Error> {
        Ok(Self {
            draw: *request
                .get_library()
                .get_symbol("conn_draw", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            definition: *request
                .get_library()
                .get_symbol("conn_def", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            properties: *request
                .get_library()
                .get_symbol("conn_props", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            serialize: *request
                .get_library()
                .get_symbol("conn_serialize", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            deserialize: *request
                .get_library()
                .get_symbol("conn_deserialize", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            default_value: *request
                .get_library()
                .get_symbol("conn_default", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
            drop_mem: *request
                .get_library()
                .get_symbol("conn_drop", request.get_path())
                .map_err(destructor::Error::from_get_symbol)?,
        })
    }
}
