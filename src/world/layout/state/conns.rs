use std::collections::HashMap;

use crate::{
    common::world::ComponentId,
    world::layout::{component::LayoutConn, requests::DestructedConnHandles},
};

/// all the connection layout in the layout world
pub struct WorldStateConns {
    handles: DestructedConnHandles,
    conns: HashMap<ComponentId, LayoutConn>,
}

impl WorldStateConns {
    pub fn new_blank(handles: DestructedConnHandles) -> Self {
        Self {
            handles,
            conns: HashMap::new(),
        }
    }
}
