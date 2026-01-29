use crate::common::world::{Direction, GateInputSocket, Vec2};

/// the path that a wire takes
pub struct ConnPath {
    /// origin of the wire in absolute coordinates
    /// the rest of the wire will be in relative coordinates
    origin: Vec2,
    segment: ConnSegment,
}

/// a segment represents the "length" part of the wire
///
/// i.e. from the starting point of a wire to the next node
pub struct ConnSegment {
    /// list of points relative to the previous junction (including origin),
    /// this includes the first point at (0,0)
    path: Vec<ConnSubSegment>,
    /// node at the last point of the path
    next_node: Box<ConnNode>,
}

/// a straight line section of a segment
///
/// guaranteed direction will not be opposite or same as previous segment
pub struct ConnSubSegment {
    direction: Direction,
    length: f64,
}

/// a node is a point of interest
pub enum ConnNode {
    /// a junction with more outgoing segments
    Junction(ConnJunction),
    /// an input socket that the data goes into
    Socket(ConnSocket),
    /// an unconnected end
    Dangling,
}

/// guaranteed that at least 2 directions are not None (otherwise its a wire not a junction)
///
/// all wires going into the junction have different directions from the wire going into the junction
pub struct ConnJunction {
    up: Option<Box<ConnSegment>>,
    right: Option<Box<ConnSegment>>,
    down: Option<Box<ConnSegment>>,
    left: Option<Box<ConnSegment>>,
}

pub struct ConnSocket {
    input_socket: GateInputSocket,
}

impl ConnPath {
    pub fn is_empty(&self) -> bool {
        self.segment.is_empty()
    }
}

impl ConnSegment {
    pub fn is_empty(&self) -> bool {
        self.path.is_empty() && self.next_node.is_dangling()
    }
}

impl ConnSubSegment {
    pub fn is_zero_length(&self) -> bool {
        self.length == 0.0
    }
}

impl ConnNode {
    pub fn is_dangling(&self) -> bool {
        matches!(self, ConnNode::Dangling)
    }
}
