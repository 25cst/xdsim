//! Destructors for components,
//! e.g. gates, data and connections
//! they are to act as a compatability layer:
//! - their function signature is the same as the latest binding version
//! - when used to run libs of older bindings, they convert the inputs appropriately

/// This module is NOT to be made public
/// content in this module should only be accessed through the corresponding
/// struct in ./
mod v0;

mod conn;
pub use conn::*;
mod data;
pub use data::*;
mod gate;
pub use gate::*;
