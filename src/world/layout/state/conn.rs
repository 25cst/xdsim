use std::collections::HashMap;

use crate::{
    common::world::{
        ComponentId, ComponentIdIncrementer, GateConsumerSocket, GateProducerSocket, Vec2,
    },
    world::{
        layout::{self, LayoutConn},
        sim,
    },
};

pub struct WorldStateConns {
    conns: HashMap<ComponentId, LayoutConn>,
}

impl WorldStateConns {
    fn get_conn_mut(
        &mut self,
        conn_id: &ComponentId,
    ) -> Result<&mut LayoutConn, Box<layout::Error>> {
        match self.conns.get_mut(conn_id) {
            Some(conn) => Ok(conn),
            None => Err(layout::Error::ConnNotFound { conn: *conn_id }.into()),
        }
    }
}

impl WorldStateConns {
    pub fn new_blank() -> Self {
        Self {
            conns: HashMap::new(),
        }
    }

    // IMPORTANT/TODO: any action that involves binding to a
    // gate socket should also add an entry in LayoutGate
}
