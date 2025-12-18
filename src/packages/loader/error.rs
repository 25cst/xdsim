use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    /// Failed to load library to memory
    LoadLib {
        /// Error message (e.g. no file at path)
        reason: String,
        /// Path to library that failed
        lib_path: PathBuf,
    },
    /// Failed to get a symbol from library
    GetSymbol {
        /// Error message
        reason: String,
        /// Name of invalid symbol (e.g. missing symbol)
        symbol_name: String,
        /// Path to library that failed
        lib_path: PathBuf,
    },
}

impl Error {
    /// Create a LoadLib error
    pub fn from_load_lib(err: libloading::Error, lib_path: PathBuf) -> Self {
        Self::LoadLib {
            reason: err.to_string(),
            lib_path,
        }
    }

    /// Create a GetSymbol error
    pub fn from_get_symbol(err: libloading::Error, symbol_name: String, lib_path: PathBuf) -> Self {
        Self::GetSymbol {
            reason: err.to_string(),
            symbol_name,
            lib_path,
        }
    }
}
