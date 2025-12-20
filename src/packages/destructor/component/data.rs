use xdsim_cbinds::{
    common::Slice,
    v0::component::{Data, DataMut},
};

use crate::{
    common::world::{DataPtr, DataPtrMut},
    packages::{
        destructor::{self, DestructRequest, component::v0},
        loader::LibraryHandle,
    },
};

/// Destructs a library into data functions
///
/// Note: a copy of library is held for the functions to remain valid
pub struct DestructedData {
    _library: LibraryHandle,
    handle: DestructedDataHandle,
}

pub enum DestructedDataHandle {
    V0(v0::DestructedData),
}

impl DestructedData {
    pub fn new(request: DestructRequest) -> Result<Self, destructor::Error> {
        let get_schema_version: fn() -> u32 = *request
            .get_library()
            .get_symbol("schema_version", request.get_path())
            .map_err(destructor::Error::from_get_symbol)?;

        let handle = match get_schema_version() {
            0 => DestructedDataHandle::V0(v0::DestructedData::new(&request)?),
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

    /// this is guaranteed to succeed
    /// (unless the component file throws an error)
    pub fn serialize(&self, data: DataPtr) -> Slice {
        match &self.handle {
            DestructedDataHandle::V0(handle) => (handle.serialize)(data),
        }
    }

    /// if deserialize fails, returns a None
    /// DataMut is guaranteed to be not null
    /// (unless the component file throws an error)
    pub fn deserialize(&self, bytes: &Slice) -> Option<DataPtrMut> {
        match &self.handle {
            DestructedDataHandle::V0(handle) => {
                let ptr = (handle.deserialize)(bytes);
                if ptr.is_null() { None } else { Some(ptr) }
            }
        }
    }

    /// this is guaranteed to succeed
    /// (unless the component file throws an error)
    pub fn default_value(&self) -> DataPtrMut {
        match &self.handle {
            DestructedDataHandle::V0(handle) => (handle.default_value)(),
        }
    }

    /// this is guaranteed to succeed
    /// it is important for the pointer to be valid
    /// otherwise this will lead to a double free or segfault
    /// (unless the component file throws an error, or the data is already been dropped)
    pub fn drop_mem(&self, data: DataPtrMut) {
        match &self.handle {
            DestructedDataHandle::V0(handle) => (handle.drop_mem)(data),
        }
    }
}
