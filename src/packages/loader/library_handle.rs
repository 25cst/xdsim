use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use libloading::{Library, Symbol};

#[derive(Clone)]
pub struct LibraryHandle {
    library: Rc<Library>,
}

impl LibraryHandle {
    pub fn new(library: Library) -> Self {
        Self {
            library: Rc::new(library),
        }
    }

    pub fn get_symbol<T>(
        &'_ self,
        symbol: &str,
        lib_path: &Path,
    ) -> Result<Symbol<'_, T>, super::Error> {
        match unsafe { self.library.get(symbol) } {
            Ok(sym) => Ok(sym),
            Err(e) => Err(super::Error::from_get_symbol(
                e,
                symbol.to_string(),
                lib_path.to_path_buf(),
            )),
        }
    }
}
