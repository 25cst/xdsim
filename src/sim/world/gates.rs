use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    common::world::{ComponentId, ComponentIdIncrementer, ComponentVersion, GateOutputSocket},
    packages::destructor::DestructedGate,
    sim::{
        self,
        component::SimGate,
        error::TickAllErrorEntry,
        requests::DestructedGateHandles,
        world::{data::WorldStateData, *},
    },
};

pub struct WorldStateGates {
    /// all gate types
    handles: DestructedGateHandles,

    /// all gates in world
    gates: HashMap<ComponentId, SimGate>,
}

impl WorldStateGates {
    /// create world state gates with only handles and no gates in world
    pub fn new_blank(handles: DestructedGateHandles) -> Self {
        Self {
            handles,
            gates: HashMap::new(),
        }
    }

    /// create a new gate in world with default state
    pub fn create_default_gate(
        &mut self,
        gate: ComponentVersion,
        world_data: &WorldStateData,
        id_counter: &mut ComponentIdIncrementer,
    ) -> Result<ComponentId, Box<sim::Error>> {
        fn get_handle<'a>(
            handles: &'a DestructedGateHandles,
            gate: &ComponentVersion,
        ) -> Option<&'a Rc<DestructedGate>> {
            handles
                .get(&gate.package)?
                .get(&gate.version)?
                .get(&gate.component)
        }

        let handle = match get_handle(&self.handles, &gate) {
            Some(found) => found,
            None => return Err(sim::Error::GateTypeNotFound { gate_type: gate }.into()),
        };

        let created_gate = SimGate::new_default(handle.clone(), world_data)?;
        let new_gate_id = id_counter.get();

        self.gates.insert(new_gate_id, created_gate);
        Ok(new_gate_id)
    }

    /// - Registers a new output (thus creating a never-before-existed buffer)
    /// - By index: the index of the output in the definition array
    pub fn register_new_output_by_index(
        &mut self,
        world_data: &mut WorldStateData,
        gate_output_socket: GateOutputSocket,
        id_counter: &mut ComponentIdIncrementer,
    ) -> Result<ComponentId, Box<sim::Error>> {
        match self.gates.get_mut(gate_output_socket.get_id()) {
            Some(gate) => gate.register_new_output(world_data, gate_output_socket, id_counter),
            None => Err(sim::Error::GateNotFound {
                gate_id: *gate_output_socket.get_id(),
            }
            .into()),
        }
    }

    /// DANGER! The output id is not checked, if it does not exist the output will not be used
    ///
    /// - Registers an output given a buffer id (outputs to an existing buffer)
    /// - By index: the index of the output in the definition array
    pub fn register_existing_output_by_index(
        &mut self,
        world_data: &mut WorldStateData,
        gate_output_socket: GateOutputSocket,
        target_buffer_id: ComponentId,
    ) -> Result<(), Box<sim::Error>> {
        match self.gates.get_mut(gate_output_socket.get_id()) {
            Some(gate) => {
                gate.register_existing_output(world_data, gate_output_socket, target_buffer_id)
            }
            None => Err(sim::Error::GateNotFound {
                gate_id: *gate_output_socket.get_id(),
            }
            .into()),
        }
    }

    /// - Unregister an output: the output is not longer connected to a buffer
    /// - By index: the index of the output in the definition array
    ///
    /// Returns the ID of the buffer that the gate originally outputs to
    pub fn unregister_output_by_index(
        &mut self,
        world_data: &mut WorldStateData,
        gate_output_socket: &GateOutputSocket,
    ) -> Result<ComponentId, Box<sim::Error>> {
        match self.gates.get_mut(gate_output_socket.get_id()) {
            Some(gate) => gate.unregister_output(world_data, gate_output_socket),
            None => Err(sim::Error::GateNotFound {
                gate_id: *gate_output_socket.get_id(),
            }
            .into()),
        }
    }

    // strictly speaking the compiler doesnt require this to SimGate::tick to be mut
    // but I've marked it as so because it would make sense
    // if it is causing trouble, we can remove it
    pub fn tick_all(&mut self, world_data: &mut WorldStateData) -> Result<(), Box<sim::Error>> {
        let mut tick_errors = Vec::new();

        for (gate_id, gate) in self.gates.iter_mut() {
            if let Err(e) = gate.tick(world_data, gate_id) {
                tick_errors.push(TickAllErrorEntry::new(*gate_id, *e));
            }
        }

        if tick_errors.is_empty() {
            Ok(())
        } else {
            Err(sim::Error::TickallErrors {
                errors: tick_errors,
            }
            .into())
        }
    }
}
