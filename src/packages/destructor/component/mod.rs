//! Destructors for components,
//! e.g. gates, data and connections

/// This module is NOT to be made public
/// content in this module should only be accessed through the corresponding
/// struct in ./
mod v0;

mod conn;
pub use conn::DestructedConnection;
mod data;
pub use data::DestructedData;
mod gate;
pub use gate::DestructedGate;
