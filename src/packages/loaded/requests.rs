use std::path::{Path, PathBuf};

use crate::packages::loader::LibraryHandle;

pub struct DestructRequest {
    library: LibraryHandle,
    lib_path: PathBuf,
}

impl DestructRequest {
    pub fn get_library(&self) -> &LibraryHandle {
        &self.library
    }

    pub fn get_path(&self) -> &Path {
        &self.lib_path
    }

    pub fn into_library(self) -> LibraryHandle {
        self.library
    }

    pub fn into_path(self) -> PathBuf {
        self.lib_path
    }
}
