use crate::packages::{
    destructor::{self, DestructRequest, component::v0},
    loader::LibraryHandle,
};

/// Destructs a library into connection functions
///
/// Note: a copy of library is held for the functions to remain valid
pub struct DestructedConn {
    _library: LibraryHandle,
    handle: DestructedConnHandle,
}

pub enum DestructedConnHandle {
    V0(v0::DestructedConn),
}

impl DestructedConn {
    pub fn new(request: DestructRequest) -> Result<Self, destructor::Error> {
        let get_schema_version: fn() -> u32 = *request
            .get_library()
            .get_symbol("schema_version")
            .map_err(destructor::Error::from_get_symbol)?;

        let handle = match get_schema_version() {
            0 => DestructedConnHandle::V0(v0::DestructedConn::new(&request)?),
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
