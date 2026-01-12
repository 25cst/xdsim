use crate::common::world::ComponentId;

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
