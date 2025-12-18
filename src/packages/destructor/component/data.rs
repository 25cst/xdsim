use crate::packages::{
    destructor::{self, DestructRequest, component::v0},
    loader::LibraryHandle,
};

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
}
