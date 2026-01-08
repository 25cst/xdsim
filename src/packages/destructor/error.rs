use std::{fmt::Display, path::PathBuf};

use crate::{common::world::ComponentVersion, packages::loader};

#[derive(Debug)]
pub enum Error {
    /// GetSymbol error when loading library
    GetSymbol {
        reason: String,
        symbol_name: String,
        lib_path: PathBuf,
    },
    /// Implementation error: except one type of error only but got another
    UnexpectedType { expected: &'static str, got: String },
    /// Definition schema not supported
    UnsupportedSchemaVersion { version: u32 },
    /// Version request cannot be parsed (because of invalid format)
    InvalidVersionReq {
        component: Box<ComponentVersion>, // to avoid err variant being too large
        version: String,
        reason: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
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
                got: e.to_string(),
            },
        }
    }
}
