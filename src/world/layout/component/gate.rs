use std::collections::HashSet;

use crate::{
    common::world::{ComponentId, GateConsumerSocket, GateProducerSocket, Rotation, Vec2},
    world::{layout, sim::SimGate},
};

/// layout gate does not include the actual gate struct,
/// that is contained in SimGate
pub struct LayoutGate {
    position: Vec2,

    rotation: Rotation,

    consumers: Vec<LayoutGateConsumerEntry>,
    producers: Vec<LayoutGateProducerEntry>,
}

impl LayoutGate {
    pub fn new(position: Vec2, gate: &SimGate) -> Self {
        let def = gate.get_def();

        Self {
            position,
            consumers: def
                .consumers
                .iter()
                .map(|entry| LayoutGateConsumerEntry {
                    rel_position: entry.position.into(),
                    bounded_conn: None,
                })
                .collect(),
            producers: def
                .producers
                .iter()
                .map(|entry| LayoutGateProducerEntry {
                    rel_position: entry.position.into(),
                    bounded_conn: HashSet::new(),
                })
                .collect(),
            rotation: Rotation::zero(),
        }
    }

    pub fn get_pos(&self) -> Vec2 {
        self.position
    }

    pub fn get_rotation(&self) -> Rotation {
        self.rotation
    }

    /// get relative position of a consumer socket
    pub fn get_consumer_rel_pos(
        &self,
        consumer_socket: &GateConsumerSocket,
    ) -> Result<Vec2, Box<layout::Error>> {
        match self.consumers.get(consumer_socket.get_index()) {
            Some(entry) => Ok(entry.rel_position),
            None => Err(layout::Error::ConsumerSocketNotFound {
                socket: *consumer_socket,
            }
            .into()),
        }
    }

    /// get relative position of a producer socket
    pub fn get_producer_rel_pos(
        &self,
        producer_socket: &GateProducerSocket,
    ) -> Result<Vec2, Box<layout::Error>> {
        match self.producers.get(producer_socket.get_index()) {
            Some(entry) => Ok(entry.rel_position),
            None => Err(layout::Error::ProducerSocketNotFound {
                socket: *producer_socket,
            }
            .into()),
        }
    }

    /// bind a point in a layout conn to a consumer socket
    ///
    /// requires the consumer socket to not be bound to anything
    pub fn point_bind_consumer(
        &mut self,
        consumer_socket: &GateConsumerSocket,
        conn_point: ComponentId,
    ) -> Result<(), Box<layout::Error>> {
        let entry = self
            .consumers
            .get_mut(consumer_socket.get_index())
            .ok_or_else(|| {
                Box::new(layout::Error::ConsumerSocketNotFound {
                    socket: *consumer_socket,
                })
            })?;

        if entry.bounded_conn.is_some() {
            return Err(layout::Error::DoubleBindConsumer { point: conn_point }.into());
        }

        entry.bounded_conn = Some(conn_point);
        Ok(())
    }

    /// bind a point in a layout conn to a producer socket
    pub fn point_bind_producer(
        &mut self,
        producer_socket: &GateProducerSocket,
        conn_point: ComponentId,
    ) -> Result<(), Box<layout::Error>> {
        let entry = self
            .producers
            .get_mut(producer_socket.get_index())
            .ok_or_else(|| {
                Box::new(layout::Error::ProducerSocketNotFound {
                    socket: *producer_socket,
                })
            })?;

        entry.bounded_conn.insert(conn_point);
        Ok(())
    }
}

pub struct LayoutGateConsumerEntry {
    rel_position: Vec2,
    bounded_conn: Option<ComponentId>,
}

pub struct LayoutGateProducerEntry {
    rel_position: Vec2,
    bounded_conn: HashSet<ComponentId>,
}
