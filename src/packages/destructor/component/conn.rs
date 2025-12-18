use crate::packages::{
    destructor::{self, DestructRequest, component::v0},
    loader::LibraryHandle,
};

pub struct DestructedConnection {
    _library: LibraryHandle,
    handle: DestructedConnectionHandle,
}

pub enum DestructedConnectionHandle {
    V0(v0::DestructedConnection),
}

impl DestructedConnection {
    pub fn new(request: DestructRequest) -> Result<Self, destructor::Error> {
        let get_schema_version: fn() -> u32 = *request
            .get_library()
            .get_symbol("schema_version", request.get_path())
            .map_err(destructor::Error::from_get_symbol)?;

        let handle = match get_schema_version() {
            0 => DestructedConnectionHandle::V0(v0::DestructedConnection::new(&request)?),
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
