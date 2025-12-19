//! Aliases to otherwise meaningless types
use std::ffi::c_void;

pub type DataPtr = *const c_void;
pub type DataPtrMut = *mut c_void;

pub type ConnectionPtr = *const c_void;
pub type ConnectionPtrMut = *mut c_void;

pub type GatePtr = *const c_void;
pub type GatePtrMut = *mut c_void;
