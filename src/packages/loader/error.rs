use std::{fmt::Display, path::PathBuf};

use semver::Version;

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
    /// Missing package from index
    MissingPackage {
        name: String,
    },
    /// Missing package version from index
    MissingPackageVersion {
        name: String,
        version: Version,
    },
    LoadAllComponentPackages {
        errors: Vec<Self>,
    },
    #[allow(clippy::enum_variant_names)]
    DestructorError {
        content: String,
    },
    DestructAllComponentPackages {
        errors: Vec<Self>,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
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
