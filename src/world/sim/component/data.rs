use std::rc::Rc;

use crate::{
    common::world::{DataPtr, DataPtrMut},
    packages::destructor::DestructedData,
};

/// A piece of simulation state data
/// - automatically drops data when self is dropped
/// - is always valid
pub struct SimData {
    handle: Rc<DestructedData>,
    data_ptr: DataPtrMut,
}

impl SimData {
    /// Create a simulation state data with its default value
    pub fn new_default(handle: Rc<DestructedData>) -> Self {
        Self {
            data_ptr: handle.default_value(),
            handle,
        }
    }

    /// Create a simulation state data with a given value
    pub fn new_with_value(handle: Rc<DestructedData>, data: DataPtrMut) -> Self {
        Self {
            handle,
            data_ptr: data,
        }
    }

    /// # Safety
    ///
    /// Using the pointer irresponsibly will cause hard to debug memory issues
    ///
    /// Get pointer to the underlying data
    pub fn get_data_ptr(&self) -> DataPtr {
        self.data_ptr
    }
}

impl Drop for SimData {
    fn drop(&mut self) {
        self.handle.drop_mem(self.data_ptr);
    }
}
