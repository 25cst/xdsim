use std::rc::Rc;

use crate::{
    common::world::{ComponentId, ComponentIdIncrementer, DataPtr, DataPtrMut, GatePtrMut},
    packages::{
        chelper::slice,
        destructor::{DestructedData, DestructedGate, DestructedGateDefinition},
    },
    sim::{self, component::SimData, world::WorldStateData},
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

#[derive(Clone)]
pub enum SimGateInputEntry {
    Unbound {
        handle: Rc<DestructedData>,
    },
    Bound {
        handle: Rc<DestructedData>,
        source_buffer: ComponentId,
    },
}

#[derive(Clone)]
pub struct SimGateOutputEntry {
    handle: Rc<DestructedData>,
    target_buffer: Option<ComponentId>,
}

impl SimGate {
    /// Create a new gate with its default configuation given a handle
    /// It will fail if one of the data type it references is not in world (data_handles)
    pub fn new_default(
        handle: Rc<DestructedGate>,
        world_data: &WorldStateData,
    ) -> Result<Self, sim::Error> {
        let gate_ptr = handle.default_value();
        let definition = match handle.normalised_definition(gate_ptr) {
            Ok(def) => def,
            Err(e) => {
                return Err(sim::Error::GateDefinition {
                    component: handle.id().clone(),
                    reason: e.to_string(),
                });
            }
        };

        let mut inputs = Vec::with_capacity(definition.inputs.len());

        for entry in definition.inputs.iter() {
            match world_data.request_handle(&entry.data_type_req) {
                Some(data_type) => inputs.push(SimGateInputEntry::Unbound {
                    handle: data_type.clone(),
                }),
                None => {
                    return Err(sim::Error::MissingRequestedDataType {
                        data_type: entry.data_type_req.clone(),
                    });
                }
            }
        }

        let mut outputs = Vec::with_capacity(definition.outputs.len());

        for entry in definition.outputs.iter() {
            match world_data.get_handle(&entry.data_type) {
                Some(data_type) => outputs.push(SimGateOutputEntry {
                    handle: data_type.clone(),
                    target_buffer: None,
                }),
                None => {
                    return Err(sim::Error::MissingDataType {
                        data_type: entry.data_type.clone(),
                    });
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

    /// - Registers a new output (thus creating a never-before-existed buffer)
    /// - By index: the index of the output in the definition array
    pub fn register_new_output_by_index(
        &mut self,
        output_index: usize,
        self_id: &ComponentId,
        id_counter: &mut ComponentIdIncrementer,
    ) -> Result<ComponentId, sim::Error> {
        match self.outputs.get_mut(output_index) {
            Some(entry) => match entry.target_buffer {
                Some(_) => Err(sim::Error::GateOutputDoubleRegister {
                    gate_type: self.handle.id().clone(),
                    gate_id: *self_id,
                    requested_index: output_index,
                }),
                None => {
                    let target_id = id_counter.get();
                    entry.target_buffer = Some(target_id);
                    Ok(target_id)
                }
            },
            None => Err(sim::Error::GateOutputIndexOutOfBounds {
                gate_type: self.handle.id().clone(),
                gate_id: *self_id,
                output_list_length: self.outputs.len(),
                requested_index: output_index,
            }),
        }
    }

    /// DANGER! The output id is not checked, if it does not exist the output will not be used
    ///
    /// - Registers an output given a buffer id (outputs to an existing buffer)
    /// - By index: the index of the output in the definition array
    pub fn register_existing_output_by_index(
        &mut self,
        output_index: usize,
        self_id: &ComponentId,
        output_id: ComponentId,
    ) -> Result<(), sim::Error> {
        match self.outputs.get_mut(output_index) {
            Some(entry) => match entry.target_buffer {
                Some(_) => Err(sim::Error::GateOutputDoubleRegister {
                    gate_type: self.handle.id().clone(),
                    gate_id: *self_id,
                    requested_index: output_index,
                }),
                None => {
                    entry.target_buffer = Some(output_id);
                    Ok(())
                }
            },
            None => Err(sim::Error::GateOutputIndexOutOfBounds {
                gate_type: self.handle.id().clone(),
                gate_id: *self_id,
                output_list_length: self.outputs.len(),
                requested_index: output_index,
            }),
        }
    }

    /// - Unregister an output: the output is not longer connected to a buffer
    /// - By index: the index of the output in the definition array
    pub fn unregister_output_by_index(
        &mut self,
        output_index: usize,
        self_id: &ComponentId,
    ) -> Result<(), sim::Error> {
        match self.outputs.get_mut(output_index) {
            Some(entry) => match entry.target_buffer {
                Some(_) => {
                    entry.target_buffer = None;
                    Ok(())
                }
                None => Err(sim::Error::GateOutputUnregisterNothing {
                    gate_type: self.handle.id().clone(),
                    gate_id: *self_id,
                    requested_index: output_index,
                }),
            },
            None => Err(sim::Error::GateOutputIndexOutOfBounds {
                gate_type: self.handle.id().clone(),
                gate_id: *self_id,
                output_list_length: self.outputs.len(),
                requested_index: output_index,
            }),
        }
    }

    /// if this function returns an error
    /// it is simply reporting a missing SimData that should exist
    /// a default value for that SimData is used and the world can containue as usual
    pub fn tick(
        &mut self, // doesn't need to be mut, if that is causing issues, will remove
        world_data: &mut WorldStateData,
    ) -> Result<(), sim::Error> {
        struct TempData {
            ptr: DataPtr,
            handle: Rc<DestructedData>,
        }

        // missing data holds the list of ComponentId that should exist but does not (may be
        // removed when it is shown there is no problem with the program)
        let mut missing_data = Vec::new();
        // temp data holds the list of temporary values for the data
        // so they can be dropped before the function returns
        let mut temp_data_ptrs = Vec::new();

        // creates the array of pointers to input data
        // (is it possible to reduce the amount of cloning here?)
        let input_slice = slice::from_vec_rustonly(
            self.inputs
                .iter()
                .map(|input| match input {
                    SimGateInputEntry::Bound {
                        handle,
                        source_buffer,
                    } => match world_data.read_buffer(source_buffer) {
                        Some(data) => unsafe { data.get_data_ptr() },
                        None => {
                            missing_data.push(*source_buffer);

                            // if buffer not in world, treat as unbound
                            let data_ptr = handle.default_value();
                            temp_data_ptrs.push(TempData {
                                ptr: data_ptr,
                                handle: handle.clone(),
                            });
                            data_ptr as DataPtr
                        }
                    },
                    SimGateInputEntry::Unbound { handle } => {
                        let data_ptr = handle.default_value();
                        temp_data_ptrs.push(TempData {
                            ptr: data_ptr,
                            handle: handle.clone(),
                        });
                        data_ptr as DataPtr
                    }
                })
                .collect(),
        );

        let output_slice = self.handle.tick(self.gate_ptr, input_slice);

        // drops the default data that was created
        // because they are temporary and will not be added to the world
        temp_data_ptrs
            .into_iter()
            .for_each(|TempData { ptr, handle }| handle.drop_mem(ptr as DataPtrMut));

        slice::from_slice::<DataPtrMut>(&output_slice)
            .iter()
            .zip(self.outputs.iter())
            .for_each(
                |(
                    data,
                    SimGateOutputEntry {
                        handle,
                        target_buffer,
                    },
                )| match target_buffer {
                    Some(output_target) => {
                        world_data.write_buffer(
                            *output_target,
                            SimData::new_with_value(handle.clone(), *data),
                        );
                    }
                    None => handle.drop_mem(*data),
                },
            );

        if missing_data.is_empty() {
            Ok(())
        } else {
            Err(sim::Error::MissingData {
                component_ids: missing_data,
            })
        }
    }
}

impl Drop for SimGate {
    fn drop(&mut self) {
        self.handle.drop_mem(self.gate_ptr);
    }
}
