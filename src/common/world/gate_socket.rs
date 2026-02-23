use crate::common::world::ComponentId;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// Identifier for a producer socket of a gate
/// - Id: ID of the gate
/// - index: the nth producer socket of the gate (as per definition)
pub struct GateProducerSocket {
    id: ComponentId,
    index: usize,
}

impl GateProducerSocket {
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
/// Identifier for a consumer socket of a gate
/// - Id: ID of the gate
/// - index: the nth producer socket of the gate (as per definition)
pub struct GateConsumerSocket {
    id: ComponentId,
    index: usize,
}

impl GateConsumerSocket {
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
