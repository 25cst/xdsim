use crate::{
    common::world::{ComponentId, Direction, Rotation, Vec2},
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
        fn get_rel_pos(bounding_box: &Vec2, direction: Direction, side_position: f64) -> Vec2 {
            match direction {
                Direction::Down => Vec2::new(side_position, 0.0),
                Direction::Left => Vec2::new(0.0, bounding_box.y() - side_position),
                Direction::Up => Vec2::new(bounding_box.x() - side_position, *bounding_box.y()),
                Direction::Right => Vec2::new(*bounding_box.x(), side_position),
            }
        }

        let def = gate.get_def();

        Self {
            position,
            inputs: def
                .inputs
                .iter()
                .map(|entry| LayoutGateInputEntry {
                    rel_position: get_rel_pos(
                        &def.bounding_box.into(),
                        entry.direction.into(),
                        entry.position,
                    ),
                    inbound_direction: entry.direction.into(),
                    bounded_conn: None,
                })
                .collect(),
            outputs: def
                .outputs
                .iter()
                .map(|entry| LayoutGateOutputEntry {
                    rel_position: get_rel_pos(
                        &def.bounding_box.into(),
                        entry.direction.into(),
                        entry.position,
                    ),
                    inbound_direction: entry.direction.into(),
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
    inbound_direction: Direction,
    bounded_conn: Option<ComponentId>,
}

pub struct LayoutGateOutputEntry {
    rel_position: Vec2,
    inbound_direction: Direction,
    bounded_conn: Option<ComponentId>,
}
