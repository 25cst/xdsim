use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    LoadLib {
        reason: String,
        lib_path: PathBuf,
    },
    GetSymbol {
        reason: String,
        symbol_name: String,
        lib_path: PathBuf,
    },
}

impl Error {
    pub fn from_load_lib(err: libloading::Error, lib_path: PathBuf) -> Self {
        Self::LoadLib {
            reason: err.to_string(),
            lib_path,
        }
    }

    pub fn from_get_symbol(err: libloading::Error, symbol_name: String, lib_path: PathBuf) -> Self {
        Self::GetSymbol {
            reason: err.to_string(),
            symbol_name,
            lib_path,
        }
    }
}
