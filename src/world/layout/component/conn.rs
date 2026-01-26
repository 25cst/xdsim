use std::{collections::HashSet, rc::Rc};

use crate::{
    common::world::{ComponentId, ComponentVersionReq, Vec2},
    packages::destructor::DestructedConn,
};

/// a connection (wire) in the layout world
pub struct LayoutConn {
    handle: Rc<DestructedConn>,
    /// supported data types: this is different from the actual data type.
    /// the actual type is in LayoutConn::producer
    data_type: ComponentVersionReq,

    producer: Option<ComponentId>,
    consumers: HashSet<ComponentId>,

    path: ConnPath,
}

pub struct ConnPath {
    origin: Vec2,
    segments: ConnSegment,
}

pub struct ConnSegment {
    /// list of points relative to the previous junction (including origin),
    /// this includes the first point at (0,0)
    path: Vec<Vec2>,
    /// junction at the last point of the path
    next: Option<ConnJunction>,
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
