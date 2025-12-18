use xdsim_cbinds::v0::{
    common::Slice,
    component::{Data, DataMut},
};

use crate::packages::loaded::{self, DestructRequest};

pub struct LoadedData {
    serialize: fn(Data) -> Slice,
    deserialize: fn(Slice) -> DataMut,
    default_value: fn() -> DataMut,
    drop_mem: fn(DataMut),
}

impl LoadedData {
    pub fn new(request: &DestructRequest) -> Result<Self, loaded::Error> {
        Ok(Self {
            serialize: *request
                .get_library()
                .get_symbol("data_serialize", request.get_path())
                .map_err(loaded::Error::from_get_symbol)?,
            deserialize: *request
                .get_library()
                .get_symbol("data_deserialize", request.get_path())
                .map_err(loaded::Error::from_get_symbol)?,
            default_value: *request
                .get_library()
                .get_symbol("data_default", request.get_path())
                .map_err(loaded::Error::from_get_symbol)?,
            drop_mem: *request
                .get_library()
                .get_symbol("data_drop", request.get_path())
                .map_err(loaded::Error::from_get_symbol)?,
        })
    }
}
