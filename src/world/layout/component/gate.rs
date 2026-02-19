use crate::{
    common::world::{ComponentId, Rotation, Vec2},
    world::sim::SimGate,
};

/// layout gate does not include the actual gate struct,
/// that is contained in SimGate
pub struct LayoutGate {
    position: Vec2,

    rotation: Rotation,

    consumers: Vec<LayoutGateConsumerEntry>,
    producers: Vec<LayoutGateProducerEntry>,
}

impl LayoutGate {
    pub fn new(position: Vec2, gate: &SimGate) -> Self {
        let def = gate.get_def();

        Self {
            position,
            consumers: def
                .consumers
                .iter()
                .map(|entry| LayoutGateConsumerEntry {
                    rel_position: entry.position.into(),
                    bounded_conn: None,
                })
                .collect(),
            producers: def
                .producers
                .iter()
                .map(|entry| LayoutGateProducerEntry {
                    rel_position: entry.position.into(),
                    bounded_conn: None,
                })
                .collect(),
            rotation: Rotation::zero(),
        }
    }

    pub fn get_pos(&self) -> Vec2 {
        self.position
    }

    pub fn get_rotation(&self) -> Rotation {
        self.rotation
    }
}

pub struct LayoutGateConsumerEntry {
    rel_position: Vec2,
    bounded_conn: Option<ComponentId>,
}

pub struct LayoutGateProducerEntry {
    rel_position: Vec2,
    bounded_conn: Option<ComponentId>,
}
