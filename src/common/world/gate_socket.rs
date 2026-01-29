use crate::common::world::ComponentId;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// Identifier for an output socket of a gate
/// - Id: ID of the gate
/// - index: the nth output socket of the gate (as per definition)
pub struct GateOutputSocket {
    id: ComponentId,
    index: usize,
}

impl GateOutputSocket {
    pub fn new(id: ComponentId, index: usize) -> Self {
        Self { id, index }
    }

    pub fn get_id(&self) -> &ComponentId {
        &self.id
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// Identifier for an input socket of a gate
/// - Id: ID of the gate
/// - index: the nth output socket of the gate (as per definition)
pub struct GateInputSocket {
    id: ComponentId,
    index: usize,
}

impl GateInputSocket {
    pub fn new(id: ComponentId, index: usize) -> Self {
        Self { id, index }
    }

    pub fn get_id(&self) -> &ComponentId {
        &self.id
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}
