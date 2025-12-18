//! The loader module
//! - Loads a dynamic library given a path.
//! - Wraps around libloading to provide a stable interface.

mod manager;

mod error;
pub use error::Error;

mod library_handle;
pub use library_handle::LibraryHandle;
