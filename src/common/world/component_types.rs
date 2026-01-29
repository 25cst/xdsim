//! Aliases to otherwise meaningless types
use std::ffi::c_void;

/// immutable pointer to data
pub type DataPtr = *const c_void;
/// mutable pointer to data
pub type DataPtrMut = *mut c_void;

/// immutable pointer to conn
pub type ConnPtr = *const c_void;
/// mutable pointer to conn
pub type ConnPtrMut = *mut c_void;

/// immutable pointer to gate
pub type GatePtr = *const c_void;
/// mutable pointer to gate
pub type GatePtrMut = *mut c_void;
