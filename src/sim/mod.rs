//! this module contains logic for the simulation view
//! in this view, the world is made of gates and buffer, on each tick:
//! - at the start of each tick, all gates reads data from their input buffers
//! - at the end of each tick, all gates puts write data to their output buffers
//! - if an input is not connected to a powered buffer (one that is connected to an output), it
//!   will assume the default value of the data

mod component;
mod error;
mod requests;
mod world;
pub use error::Error;
