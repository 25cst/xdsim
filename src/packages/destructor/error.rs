use std::path::PathBuf;

use crate::packages::loader;

pub enum Error {
    GetSymbol {
        reason: String,
        symbol_name: String,
        lib_path: PathBuf,
    },
    UnexpectedType {
        expected: &'static str,
        got: String,
    },
    UnsupportedSchemaVersion {
        version: u32,
    },
}

impl Error {
    pub fn from_get_symbol(error: loader::Error) -> Self {
        match error {
            loader::Error::GetSymbol {
                reason,
                symbol_name,
                lib_path,
            } => Self::GetSymbol {
                reason,
                symbol_name,
                lib_path,
            },
            e => Self::UnexpectedType {
                expected: "GetSymbol",
                got: format!("{e:?}"),
            },
        }
    }
}
