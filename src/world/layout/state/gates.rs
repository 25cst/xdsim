use std::collections::HashMap;

use crate::{
    common::world::{ComponentId, GateConsumerSocket, GateProducerSocket, Vec2},
    world::{
        layout::{self, component::LayoutGate},
        sim::SimGate,
    },
};

/// layout world state gate shadows over the sim world state
///
/// gate IDs satisfies constraints
/// - the key set of layout::WorldGates and sim::WorldGates are the same,
///   aka a gate exist in sim world state iff it exist in layout state
pub struct WorldStateGates {
    gates: HashMap<ComponentId, LayoutGate>,
}

impl WorldStateGates {
    /// create new layout gate state
    pub fn new_blank() -> Self {
        Self {
            gates: HashMap::new(),
        }
    }

    /// DANGER: unchecked operation, does not guarantee that the gate exist in sim world,
    /// also does not guarantee this gate is not already in hashmap
    ///
    /// adds new layout gate into layout,
    pub fn add_gate(&mut self, gate_id: ComponentId, origin: Vec2, sim_gate: &SimGate) {
        self.gates
            .insert(gate_id, LayoutGate::new(origin, sim_gate));
    }

    /// returns a layout gate
    pub fn get_gate(&self, gate_id: &ComponentId) -> Result<&LayoutGate, Box<layout::Error>> {
        self.gates
            .get(gate_id)
            .ok_or_else(|| Box::new(layout::Error::GateNotFound { gate: *gate_id }))
    }

    /// returns a mutable reference to layout gate
    pub fn get_gate_mut(
        &mut self,
        gate_id: &ComponentId,
    ) -> Result<&mut LayoutGate, Box<layout::Error>> {
        self.gates
            .get_mut(gate_id)
            .ok_or_else(|| Box::new(layout::Error::GateNotFound { gate: *gate_id }))
    }

    /// bind a point in a layout conn to a consumer socket
    ///
    /// requires the consumer socket to not be bound to anything
    pub fn point_bind_consumer(
        &mut self,
        consumer_socket: &GateConsumerSocket,
        conn_point: ComponentId,
    ) -> Result<(), Box<layout::Error>> {
        self.get_gate_mut(consumer_socket.get_id())?
            .point_bind_consumer(consumer_socket, conn_point)
    }

    /// bind a point in a layout conn to a producer socket
    ///
    /// USING THIS DIRECTLY WILL LEAVE THE WORLD INCONSISTENT, this function is only meant
    /// to be used by LayoutConn
    pub fn point_bind_producer(
        &mut self,
        producer_socket: &GateProducerSocket,
        conn_point: ComponentId,
    ) -> Result<(), Box<layout::Error>> {
        self.get_gate_mut(producer_socket.get_id())?
            .point_bind_producer(producer_socket, conn_point)
    }
}
