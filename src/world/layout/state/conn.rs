use std::collections::HashMap;

use crate::{
    common::world::{ComponentId, ComponentIdIncrementer, GateProducerSocket, Vec2},
    world::{
        layout::{self, ConnDrawDanglingRes, ConnDrawNewRes, LayoutConn},
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

    /// draw a new connection from a producer socket to a new point
    pub fn draw_new(
        &mut self,
        counter: &mut ComponentIdIncrementer,
        sim_world: &sim::WorldState,
        layout_gates: &layout::WorldStateGates,
        from: GateProducerSocket,
        to: Vec2,
    ) -> Result<ConnDrawNewRes, Box<layout::Error>> {
        let conn_id = counter.get(crate::common::world::ComponentIdType::Conn);
        let new_conn = LayoutConn::draw_new(conn_id, counter, sim_world, layout_gates, from, to)?;

        self.conns.insert(conn_id, new_conn.conn);

        Ok(ConnDrawNewRes {
            conn_id,
            segment_id: new_conn.segment_id,
            producer_point: new_conn.producer_point,
            dangling_point: new_conn.dangling_point,
        })
    }

    /// draw a new segment from a dangling point of a conn
    pub fn draw_dangling(
        &mut self,
        counter: &mut ComponentIdIncrementer,
        conn_id: ComponentId,
        from: ComponentId,
        to: Vec2,
    ) -> Result<ConnDrawDanglingRes, Box<layout::Error>> {
        let conn = self.get_conn_mut(&conn_id)?;
        let res = conn.draw_dangling(conn_id, counter, from, to)?;

        Ok(ConnDrawDanglingRes {
            segment_id: res.segment_id,
            dangling_point: res.dangling_point,
        })
    }
}
