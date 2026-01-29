use std::{collections::HashSet, rc::Rc};

use crate::{
    common::world::{ComponentId, ComponentVersionReq, Vec2},
    packages::destructor::DestructedConn,
    world::layout::component::conn_path::ConnPath,
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
