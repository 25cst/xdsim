use std::{
    env::consts::DLL_EXTENSION,
    path::{Path, PathBuf},
};

use libloading::Library;

use crate::packages::loader::library_handle::LibraryHandle;

/// Helper for loading libraries from a folder
pub struct LoadManager {
    root: PathBuf,
}

impl LoadManager {
    pub fn new<T: Into<PathBuf>>(root: T) -> Self {
        Self { root: root.into() }
    }

    pub fn get_path(&self, path: &str) -> PathBuf {
        self.root.join(path).with_extension(DLL_EXTENSION)
    }

    pub fn load(lib_path: &Path) -> Result<LibraryHandle, super::Error> {
        match unsafe { Library::new(lib_path) } {
            Ok(lib) => Ok(LibraryHandle::new(lib)),
            Err(e) => Err(super::Error::from_load_lib(e, lib_path.to_path_buf())),
        }
    }
}
