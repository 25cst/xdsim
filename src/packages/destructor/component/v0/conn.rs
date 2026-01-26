use xdsim_cbinds::{
    common::Slice,
    v0::{
        app_state::PropertiesMut,
        component::{Conn, ConnDefinition, ConnMut, ConnSegment, Data},
        graphics::Graphic,
    },
};

use crate::packages::destructor::{self, DestructRequest};

pub struct DestructedConn {
    pub draw: extern "C" fn(Conn, *const ConnSegment, Data) -> Graphic,
    pub definition: extern "C" fn(Conn) -> ConnDefinition,
    pub properties: extern "C" fn(ConnMut) -> PropertiesMut,
    pub serialize: extern "C" fn(Conn) -> Slice,
    pub deserialize: extern "C" fn(*const Slice) -> ConnMut,
    pub default_value: extern "C" fn() -> ConnMut,
    pub drop_mem: extern "C" fn(ConnMut),
}

impl DestructedConn {
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
