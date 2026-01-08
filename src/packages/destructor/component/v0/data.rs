use xdsim_cbinds::{
    common::Slice,
    v0::component::{Data, DataMut},
};

use crate::packages::destructor::{self, DestructRequest};

pub struct DestructedData {
    pub serialize: fn(Data) -> Slice,
    pub deserialize: fn(*const Slice) -> DataMut,
    pub default_value: fn() -> DataMut,
    pub drop_mem: fn(DataMut),
}

impl DestructedData {
    pub fn new(request: &DestructRequest) -> Result<Self, destructor::Error> {
        Ok(Self {
            serialize: *request
                .get_library()
                .get_symbol("data_serialize")
                .map_err(destructor::Error::from_get_symbol)?,
            deserialize: *request
                .get_library()
                .get_symbol("data_deserialize")
                .map_err(destructor::Error::from_get_symbol)?,
            default_value: *request
                .get_library()
                .get_symbol("data_default")
                .map_err(destructor::Error::from_get_symbol)?,
            drop_mem: *request
                .get_library()
                .get_symbol("data_drop")
                .map_err(destructor::Error::from_get_symbol)?,
        })
    }
}
