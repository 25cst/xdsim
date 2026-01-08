use xdsim_cbinds::{
    common::Slice,
    v0::{
        app_state::PropertiesMut,
        component::{Connection, ConnectionDefinition, ConnectionMut, ConnectionSegment, Data},
        graphics::Graphic,
    },
};

use crate::packages::destructor::{self, DestructRequest};

pub struct DestructedConnection {
    pub draw: fn(Connection, *const ConnectionSegment, Data) -> Graphic,
    pub definition: fn(Connection) -> ConnectionDefinition,
    pub properties: fn(ConnectionMut) -> PropertiesMut,
    pub serialize: fn(Connection) -> Slice,
    pub deserialize: fn(*const Slice) -> ConnectionMut,
    pub default_value: fn() -> ConnectionMut,
    pub drop_mem: fn(ConnectionMut),
}

impl DestructedConnection {
    pub fn new(request: &DestructRequest) -> Result<Self, destructor::Error> {
        Ok(Self {
            draw: *request
                .get_library()
                .get_symbol("conn_draw")
                .map_err(destructor::Error::from_get_symbol)?,
            definition: *request
                .get_library()
                .get_symbol("conn_def")
                .map_err(destructor::Error::from_get_symbol)?,
            properties: *request
                .get_library()
                .get_symbol("conn_props")
                .map_err(destructor::Error::from_get_symbol)?,
            serialize: *request
                .get_library()
                .get_symbol("conn_serialize")
                .map_err(destructor::Error::from_get_symbol)?,
            deserialize: *request
                .get_library()
                .get_symbol("conn_deserialize")
                .map_err(destructor::Error::from_get_symbol)?,
            default_value: *request
                .get_library()
                .get_symbol("conn_default")
                .map_err(destructor::Error::from_get_symbol)?,
            drop_mem: *request
                .get_library()
                .get_symbol("conn_drop")
                .map_err(destructor::Error::from_get_symbol)?,
        })
    }
}
