use std::collections::HashMap;

use crate::{
    common::world::ComponentId,
    world::layout::{component::LayoutConn, requests::DestructedConnHandles},
};

pub struct WorldStateConns {
    handles: DestructedConnHandles,
    conns: HashMap<ComponentId, LayoutConn>,
}
