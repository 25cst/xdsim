//! The destructor module
//! - Destructs a library into a struct containing all functions of interest in the library

mod component;
mod error;
pub use error::Error;
mod requests;
pub use requests::*;
