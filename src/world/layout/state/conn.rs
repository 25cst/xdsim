use std::collections::HashMap;

use crate::{
    common::world::{
        ComponentId, ComponentIdIncrementer, ComponentIdType, GateConsumerSocket,
        GateProducerSocket, Vec2,
    },
    world::{
        layout::{self, LayoutConn, LayoutConnDrawDanglingRes, LayoutConnDrawRes},
        sim,
    },
};

/// returned new connection, this sturct only exist to be destructed
pub struct LayoutNewConnRes {
    pub from: ComponentId,
    pub to: ComponentId,
    pub segment: ComponentId,
    pub conn: ComponentId,
}

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

    /// draw a new segment from a point to a position,
    /// creating a new point in that position
    pub fn draw_dangling(
        &mut self,
        counter: &mut ComponentIdIncrementer,
        conn_id: ComponentId,
        from: ComponentId,
        to: Vec2,
    ) -> Result<LayoutConnDrawDanglingRes, Box<layout::Error>> {
        let conn = self.get_conn_mut(&conn_id)?;
        conn.draw_dangling(conn_id, counter, from, to)
    }

    /// draw a new connection
    pub fn draw_new(
        &mut self,
        sim_world: &mut sim::WorldState,
        layout_gates: &mut layout::WorldStateGates,
        from: GateProducerSocket,
        to: Vec2,
    ) -> Result<LayoutNewConnRes, Box<layout::Error>> {
        let conn_id = sim_world.counter_mut().get(ComponentIdType::Conn);
        let res =
            LayoutConn::draw_new(conn_id, sim_world, layout_gates, from, to).inspect_err(|_| {
                let _ = sim_world.counter_mut().unregister(&conn_id);
            })?;
        self.conns.insert(conn_id, res.conn);

        Ok(LayoutNewConnRes {
            to: res.to,
            segment: res.segment,
            from: res.from,
            conn: conn_id,
        })
    }
}
