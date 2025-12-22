use std::{collections::HashMap, rc::Rc};

use xdsim_cbinds::v0::component::{DataMut, GateDefinition};

use crate::{
    common::world::{ComponentId, ComponentLibMinorId, DataPtr, DataPtrMut, GatePtrMut},
    packages::{
        chelper::slice,
        destructor::{DestructedData, DestructedGate, DestructedGateIOEntry},
    },
    sim::{self, component::SimData, sim_world::WorldStateData},
};

/// A single gate
/// - calls drop_mem on itself when dropped
pub struct SimGate {
    handle: Rc<DestructedGate>,
    gate_ptr: GatePtrMut,

    input_sources: Vec<SimGateIOEntry>,
    output_targets: Vec<SimGateIOEntry>,
}

/// an IO entry struct that contains the handle to the Data
/// and the buffer the data is to be fetched from/put onto
/// buffer_id is none if it is not connected to an input/output wire for that gate IO
struct SimGateIOEntry {
    pub handle: Rc<DestructedData>,
    pub buffer_id: Option<ComponentId>,
}

impl SimGate {
    /// Create a new gate with its default configuation given a handle
    /// It will fail if one of the data type it references is not in world (data_handles)
    pub fn new_default(
        handle: Rc<DestructedGate>,
        world_data: &WorldStateData,
    ) -> Result<Self, sim::Error> {
        let gate_ptr = handle.default_value();
        let definition = handle.normalised_definition(gate_ptr);

        fn to_simgate_io_entries(
            destructed_io_entries: Vec<DestructedGateIOEntry>,
            world_data: &WorldStateData,
        ) -> Result<Vec<SimGateIOEntry>, sim::Error> {
            let mut simgate_io_entries = Vec::with_capacity(destructed_io_entries.len());
            for DestructedGateIOEntry {
                data_type,
                name: _,
                position: _,
            } in destructed_io_entries
            {
                let data_type = data_type.into_minor();
                simgate_io_entries.push(SimGateIOEntry {
                    handle: match world_data.get_handle(&data_type) {
                        Some(destructed_lib) => destructed_lib.clone(),
                        None => {
                            return Err(sim::Error::MissingDataType {
                                data_ident: data_type,
                            });
                        }
                    },
                    buffer_id: None,
                });
            }

            Ok(simgate_io_entries)
        }

        Ok(Self {
            input_sources: to_simgate_io_entries(definition.inputs, world_data)?,
            output_targets: to_simgate_io_entries(definition.outputs, world_data)?,
            gate_ptr,
            handle,
        })
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
            self.input_sources
                .iter()
                .map(|SimGateIOEntry { handle, buffer_id }| match buffer_id {
                    Some(buffer_id) => match world_data.read_buffer(buffer_id) {
                        Some(data) => unsafe { data.get_data_ptr() },
                        None => {
                            missing_data.push(*buffer_id);
                            let data_ptr = handle.default_value();
                            temp_data_ptrs.push(TempData {
                                ptr: data_ptr,
                                handle: handle.clone(),
                            });
                            data_ptr as DataPtr
                        }
                    },
                    None => {
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
            .zip(self.output_targets.iter())
            .for_each(
                |(data, SimGateIOEntry { handle, buffer_id })| match buffer_id {
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
