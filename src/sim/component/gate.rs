use std::rc::Rc;

use crate::{common::world::GatePtrMut, packages::destructor::DestructedGate};

pub struct SimGate {
    handle: Rc<DestructedGate>,
    gate_ptr: GatePtrMut,
}

/*
impl SimGate {
    /// Create a simulation state gate with its default value
    pub fn new_default(handle: Rc<DestructedGate>) -> Self {
        Self {
            gate_ptr: handle.default_value(),
            handle,
        }
    }
}
*/
