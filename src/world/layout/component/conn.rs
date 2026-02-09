use std::collections::{HashMap, HashSet};

use crate::{
    common::world::{ComponentId, ComponentIdIncrementer, ComponentIdType, GateOutputSocket, Vec2},
    world::layout,
};

/// collection of points and segments with constraints
pub struct LayoutConn {
    points: HashMap<ComponentId, LayoutConnPoint>,
    segments: HashMap<ComponentId, LayoutConnSegment>,
    /// the data producer the conn is connected to
    producer: Option<GateOutputSocket>,
    /// the data consumers the conn is connected to
    consumers: HashSet<GateOutputSocket>,
}

impl LayoutConn {
    fn make_point(
        &mut self,
        self_id: ComponentId,
        counter: &mut ComponentIdIncrementer,
        pos: Vec2,
    ) -> ComponentId {
        let id = counter.get(ComponentIdType::ConnPoint { conn_id: self_id });

        self.points.insert(
            id,
            LayoutConnPoint {
                pos,
                before: LayoutConnPointBefore::Dangling,
                segments_after: HashSet::new(),
                consumer: None,
            },
        );

        id
    }

    fn make_segment(
        &mut self,
        self_id: ComponentId,
        counter: &mut ComponentIdIncrementer,
        from: ComponentId,
        to: ComponentId,
    ) -> Result<ComponentId, Box<layout::Error>> {
        if !self.points.contains_key(&from) {
            return Err(layout::Error::ConnPointNotFound { point: from }.into());
        }

        if !self.points.contains_key(&to) {
            return Err(layout::Error::ConnPointNotFound { point: to }.into());
        }

        let id = counter.get(ComponentIdType::ConnSegment { conn_id: self_id });

        self.segments
            .insert(id, LayoutConnSegment::new_unchecked(from, to));

        Ok(id)
    }
}

impl LayoutConn {
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

/// a point
///
/// each point can only have at most one input (includes producer and segments before)
struct LayoutConnPoint {
    /// position of the layout on canvas
    pos: Vec2,
    /// the connections/input socket before this point
    before: LayoutConnPointBefore,
    /// the segments connected after the point
    segments_after: HashSet<ComponentId>,
    /// the consumer this point outputs to
    consumer: Option<GateOutputSocket>,
}

/// stuff that happens before the point
enum LayoutConnPointBefore {
    /// one output socket
    Producer { output_socket: GateOutputSocket },
    /// one segment
    Segment { segment_id: ComponentId },
    /// nothing
    Dangling,
}

/// a segment connecting between two points,
/// the from and to component IDs are points
/// that exists inside the conn
struct LayoutConnSegment {
    from: ComponentId,
    to: ComponentId,
}

impl LayoutConnSegment {
    pub fn new_unchecked(from: ComponentId, to: ComponentId) -> Self {
        Self { from, to }
    }
}
