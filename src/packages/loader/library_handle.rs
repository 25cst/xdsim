use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use libloading::{Library, Symbol};

/// Handle to a loaded dynamic library
///
/// Cloning the handle will create new handles referencing the same library
/// If the last handle to a library is dropped, the library is unloaded from memory
#[derive(Clone)]
pub struct LibraryHandle {
    library: Rc<Library>,
    path: PathBuf,
}

impl LibraryHandle {
    pub fn new(library: Library, path: PathBuf) -> Self {
        Self {
            library: Rc::new(library),
            path,
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    /// DANGER!
    /// Symbols are only valid if the library is loaded in memory
    /// Because the library will only unload when all handles are dropped,
    /// this may result in the symbol being valid for a while before becoming invalid
    ///
    /// You must store a copy of the originating LibraryHandle alongside the symbol
    /// failure to do so will likely result in segfaults
    ///
    /// Note: lib_path is shown in the error message if getting symbol failed
    pub fn get_symbol<T>(&'_ self, symbol: &str) -> Result<Symbol<'_, T>, super::Error> {
        match unsafe { self.library.get(symbol) } {
            Ok(sym) => Ok(sym),
            Err(e) => Err(super::Error::from_get_symbol(
                e,
                symbol.to_string(),
                self.path.clone(),
            )),
        }
    }
}
