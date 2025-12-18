//! The loader module
//!
//! This module is used to load a dynamic library given a path.
//!
//! It wraps around libloading to provide a stable interface.

mod manager;

mod error;
pub use error::Error;

mod library_handle;
pub use library_handle::LibraryHandle;
