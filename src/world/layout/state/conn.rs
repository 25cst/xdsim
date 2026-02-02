use std::collections::HashMap;

use crate::{common::world::ComponentId, world::layout::LayoutConn};

pub struct WorldStateConns {
    conns: HashMap<ComponentId, LayoutConn>,
}
