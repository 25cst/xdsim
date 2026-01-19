use std::{
    cell::UnsafeCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    common::world::{
        ComponentId, ComponentIdIncrementer, ComponentVersion, GateInputSocket, GateOutputSocket,
    },
    packages::destructor::DestructedGate,
    sim::{
        self,
        component::{SimData, SimGate},
        error::TickAllErrorEntry,
        requests::DestructedGateHandles,
        world::{data::WorldStateData, *},
    },
};

pub struct WorldStateGates {
    /// all gate types
    handles: DestructedGateHandles,

    /// all gates in world
    gates: HashMap<ComponentId, UnsafeCell<SimGate>>,
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

        self.gates
            .insert(new_gate_id, UnsafeCell::new(created_gate));
        Ok(new_gate_id)
    }

    // strictly speaking the compiler doesnt require this to SimGate::tick to be mut
    // but I've marked it as so because it would make sense
    // if it is causing trouble, we can remove it
    pub fn tick_all(&mut self) -> Result<(), Box<sim::Error>> {
        let mut tick_errors = Vec::new();

        for (gate_id, gate) in self.gates.iter() {
            // the only variable that will be mutated are write_only buffers
            // they will be written once only in a tick, and will not be read from
            // all other variables are to remain unchanged
            if let Err(e) = unsafe { &mut *gate.get() }.tick(&self, gate_id) {
                tick_errors.push(TickAllErrorEntry::new(*gate_id, *e));
            }
        }

        // flush is in the same funciton as tick_all, because it is ran only after ticking
        for gate in self.gates.values_mut() {
            gate.get_mut().flush();
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

    /// get the output of a socket
    pub fn get_output(&self, output_socket: &GateOutputSocket) -> Option<&SimData> {
        // unsafe ok because it is treating self as immutable
        unsafe { &*self.gates.get(output_socket.get_id())?.get() }
            .get_output(output_socket.get_index())
    }

    pub fn connect(
        &mut self,
        output_socket: GateOutputSocket,
        input_socket: GateInputSocket,
    ) -> Result<(), Box<sim::Error>> {
        // the unsafes are fine because we are modifying dependents in output gate
        // and source in input gate
        let input_gate = match self.gates.get(input_socket.get_id()) {
            Some(gate) => unsafe { &mut *gate.get() },
            None => return Err(sim::Error::InputSocketNotFound { input_socket }.into()),
        };

        let output_gate = match self.gates.get(output_socket.get_id()) {
            Some(gate) => unsafe { &mut *gate.get() },
            None => return Err(sim::Error::OutputSocketNotFound { output_socket }.into()),
        };

        input_gate.connect_input_to(
            &input_socket,
            output_socket,
            output_gate.get_output_type(&output_socket)?,
        )?;

        output_gate.output_connected_from(&output_socket, input_socket)
    }
}
