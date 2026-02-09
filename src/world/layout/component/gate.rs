use crate::{
    common::world::{ComponentId, Rotation, Vec2},
    world::sim::SimGate,
};

/// layout gate does not include the actual gate struct,
/// that is contained in SimGate
pub struct LayoutGate {
    position: Vec2,

    rotation: Rotation,

    inputs: Vec<LayoutGateInputEntry>,
    outputs: Vec<LayoutGateOutputEntry>,
}

impl LayoutGate {
    pub fn new(position: Vec2, gate: &SimGate) -> Self {
        let def = gate.get_def();

        Self {
            position,
            inputs: def
                .inputs
                .iter()
                .map(|entry| LayoutGateInputEntry {
                    rel_position: entry.position.into(),
                    bounded_conn: None,
                })
                .collect(),
            outputs: def
                .outputs
                .iter()
                .map(|entry| LayoutGateOutputEntry {
                    rel_position: entry.position.into(),
                    bounded_conn: None,
                })
                .collect(),
            rotation: Rotation::D0,
        }
    }

    pub fn set_pos(&mut self, position: Vec2) {
        self.position = position
    }
}

pub struct LayoutGateInputEntry {
    rel_position: Vec2,
    bounded_conn: Option<ComponentId>,
}

pub struct LayoutGateOutputEntry {
    rel_position: Vec2,
    bounded_conn: Option<ComponentId>,
}
