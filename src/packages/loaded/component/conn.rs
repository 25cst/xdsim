use crate::packages::{
    loaded::{self, DestructRequest, component::v0},
    loader::LibraryHandle,
};

pub struct LoadedConnection {
    _library: LibraryHandle,
    handle: LoadedConnectionHandle,
}

pub enum LoadedConnectionHandle {
    V0(v0::LoadedConnection),
}

impl LoadedConnection {
    pub fn new(request: DestructRequest) -> Result<Self, loaded::Error> {
        let get_schema_version: fn() -> u32 = *request
            .get_library()
            .get_symbol("schema_version", request.get_path())
            .map_err(loaded::Error::from_get_symbol)?;

        let handle = match get_schema_version() {
            0 => LoadedConnectionHandle::V0(v0::LoadedConnection::new(&request)?),
            unsupported_version => {
                return Err(loaded::Error::UnsupportedSchemaVersion {
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
