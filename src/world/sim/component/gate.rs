use std::{collections::HashSet, rc::Rc};

use crate::{
    common::world::{
        ComponentId, ComponentVersion, ComponentVersionReq, DataPtrMut, GateInputSocket,
        GateOutputSocket, GatePtrMut,
    },
    packages::{
        chelper::slice,
        destructor::{DestructedData, DestructedGate, DestructedGateDefinition},
    },
    world::sim::{
        self,
        component::SimData,
        state::{WorldStateData, WorldStateGates},
    },
};

/// A single gate
/// - calls drop_mem on itself when dropped
pub struct SimGate {
    handle: Rc<DestructedGate>,
    gate_ptr: GatePtrMut,

    definition: DestructedGateDefinition,

    inputs: Vec<SimGateInputEntry>,
    outputs: Vec<SimGateOutputEntry>,
}

impl SimGate {
    pub fn get_type(&self) -> &ComponentVersion {
        self.handle.id()
    }
}

#[derive(Clone)]
pub struct SimGateInputEntry {
    request: ComponentVersionReq,
    /// data type handle to use if unbound
    default_data_type: Rc<DestructedData>,
    status: SimGateInputEntryStatus,
}

#[derive(Clone)]
pub enum SimGateInputEntryStatus {
    Unbound,
    Bound {
        handle: Rc<DestructedData>,
        source: GateOutputSocket,
    },
}

pub struct SimGateOutputEntry {
    handle: Rc<DestructedData>,

    read_only: SimData,
    write_only: Option<SimData>,

    /// inputs that depend on this output socket
    dependents: HashSet<GateInputSocket>,
}

impl SimGate {
    pub fn get_output(&self, index: usize) -> Option<&SimData> {
        Some(&self.outputs.get(index)?.read_only)
    }

    /// Create a new gate with its default configuation given a handle
    /// It will fail if one of the data type it references is not in world (data_handles)
    pub fn new_default(
        handle: Rc<DestructedGate>,
        world_data: &WorldStateData,
    ) -> Result<Self, Box<sim::Error>> {
        let gate_ptr = handle.default_value();
        let definition = handle.normalised_definition(gate_ptr).map_err(|e| {
            Box::new(sim::Error::GateDefinition {
                component: handle.id().clone(),
                reason: e.to_string(),
            })
        })?;

        let mut inputs = Vec::with_capacity(definition.inputs.len());

        for entry in definition.inputs.iter() {
            match world_data.request_handle(&entry.data_type_req) {
                Some(data_type) => inputs.push(SimGateInputEntry {
                    request: entry.data_type_req.clone(),
                    default_data_type: data_type.clone(),
                    status: SimGateInputEntryStatus::Unbound,
                }),
                None => {
                    return Err(sim::Error::RequestedDataTypeNotFound {
                        data_type: entry.data_type_req.clone(),
                    }
                    .into());
                }
            }
        }

        let mut outputs = Vec::with_capacity(definition.outputs.len());

        for entry in definition.outputs.iter() {
            match world_data.get_handle(&entry.data_type) {
                Some(data_type) => outputs.push(SimGateOutputEntry {
                    handle: data_type.clone(),
                    read_only: SimData::new_default(data_type.clone()),
                    write_only: None,
                    dependents: HashSet::new(),
                }),
                None => {
                    return Err(sim::Error::DataTypeNotFound {
                        data_type: entry.data_type.clone(),
                    }
                    .into());
                }
            }
        }

        Ok(Self {
            gate_ptr,
            handle,

            inputs,
            outputs,

            definition,
        })
    }

    /// if this function returns an error
    /// it is simply reporting a missing SimData that should exist
    /// a default value for that SimData is used and the world can containue as usual
    pub fn tick(
        &mut self, // doesn't need to be mut, if that is causing issues, will remove
        world_gates: &WorldStateGates,
        self_id: &ComponentId,
    ) -> Result<(), Box<sim::Error>> {
        let mut errors = Vec::new();
        // temp data holds the list of temporary values for the data
        // so they can be dropped before the function returns
        let mut temp_datas = Vec::new();

        // creates the array of pointers to input data
        // (is it possible to reduce the amount of cloning here?)
        let input_slice = slice::from_vec_rustonly(
            self.inputs
                .iter()
                .map(|input| match &input.status {
                    SimGateInputEntryStatus::Bound { handle, source } => {
                        match world_gates.get_output(source) {
                            Some(data) => data.get_data_ptr(),
                            None => {
                                errors.push(sim::Error::OutputSocketNotFound {
                                    output_socket: *source,
                                });

                                // if output socket not in world, treat as unbound
                                let temp_data = SimData::new_default(handle.clone());
                                let ptr = temp_data.get_data_ptr();
                                temp_datas.push(temp_data);
                                ptr
                            }
                        }
                    }
                    SimGateInputEntryStatus::Unbound => {
                        let temp_data = SimData::new_default(input.default_data_type.clone());
                        let ptr = temp_data.get_data_ptr();
                        temp_datas.push(temp_data);
                        ptr
                    }
                })
                .collect(),
        );

        let output_slice = self.handle.tick(self.gate_ptr, &input_slice);

        slice::from_slice::<DataPtrMut>(&output_slice)
            .iter()
            .zip(self.outputs.iter_mut())
            .for_each(
                |(
                    &data,
                    SimGateOutputEntry {
                        handle,
                        write_only,
                        dependents: _,
                        read_only: _,
                    },
                )| {
                    *write_only = Some(SimData::new_with_value(handle.clone(), data));
                },
            );

        if errors.is_empty() {
            Ok(())
        } else {
            Err(sim::Error::TickSingleGate {
                gate_id: *self_id,
                errors,
            }
            .into())
        }
    }

    /// replace all read_only buffers with write_only buffers
    /// this is to be ran at the end of a tick
    pub fn flush(&mut self) {
        for output in self.outputs.iter_mut() {
            if let Some(new_output) = output.write_only.take() {
                output.read_only = new_output;
            }
        }
    }

    /// connect an input (of this gate) to an output (of another gate).
    /// this also checks if their types are compatible
    pub fn connect_input_to(
        &mut self,
        input_socket: &GateInputSocket,
        output_socket: GateOutputSocket,
        output_type: &Rc<DestructedData>,
    ) -> Result<(), Box<sim::Error>> {
        let input_entry = self
            .inputs
            .get_mut(input_socket.get_index())
            .ok_or_else(|| {
                Box::new(sim::Error::InputSocketNotFound {
                    input_socket: *input_socket,
                })
            })?;

        match input_entry.status {
            SimGateInputEntryStatus::Unbound => {
                if !input_entry.request.matches(output_type.id()) {
                    return Err(sim::Error::IOTypeMismatch {
                        input_socket: *input_socket,
                        output_socket,
                    }
                    .into());
                }

                input_entry.status = SimGateInputEntryStatus::Bound {
                    handle: output_type.clone(),
                    source: output_socket,
                };
                Ok(())
            }
            SimGateInputEntryStatus::Bound { source, .. } => {
                Err(sim::Error::InputSocketDoubleBound {
                    input_socket: *input_socket,
                    current_output_source: source,
                    new_output_source: output_socket,
                }
                .into())
            }
        }
    }

    /// connect an output (of this gate) to an input (of another gate)
    pub fn output_connected_from(
        &mut self,
        output_socket: &GateOutputSocket,
        input_socket: GateInputSocket,
    ) -> Result<(), Box<sim::Error>> {
        let output_entry = self
            .outputs
            .get_mut(output_socket.get_index())
            .ok_or_else(|| {
                Box::new(sim::Error::OutputSocketNotFound {
                    output_socket: *output_socket,
                })
            })?;

        if output_entry.dependents.insert(input_socket) {
            Ok(())
        } else {
            Err(sim::Error::OutputSocketDoubleBound {
                input_socket,
                output_socket: *output_socket,
            }
            .into())
        }
    }

    /// gate data type (destructed gate) of an output index of this gate,
    /// it does not check if the component id in the output socket if correct,
    /// you will have to make sure that is the case yourself
    pub fn get_output_type(
        &self,
        output_socket: &GateOutputSocket,
    ) -> Result<&Rc<DestructedData>, Box<sim::Error>> {
        match self.outputs.get(output_socket.get_index()) {
            Some(output_entry) => Ok(&output_entry.handle),
            None => Err(sim::Error::OutputSocketNotFound {
                output_socket: *output_socket,
            }
            .into()),
        }
    }
}

impl Drop for SimGate {
    fn drop(&mut self) {
        self.handle.drop_mem(self.gate_ptr);
    }
}
