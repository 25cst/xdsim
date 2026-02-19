use std::{collections::HashSet, rc::Rc};

use crate::{
    common::world::{
        ComponentId, ComponentVersion, ComponentVersionReq, DataPtrMut, GateConsumerSocket,
        GateProducerSocket, GatePtrMut,
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

    consumers: Vec<SimGateConsumerEntry>,
    producers: Vec<SimGateProducerEntry>,
}

impl SimGate {
    /// get gate type identifier
    pub fn get_type(&self) -> &ComponentVersion {
        self.handle.id()
    }
}

#[derive(Clone)]
pub struct SimGateConsumerEntry {
    request: ComponentVersionReq,
    /// data type handle to use if unbound
    default_data_type: Rc<DestructedData>,
    status: SimGateConsumerEntryStatus,
}

#[derive(Clone)]
pub enum SimGateConsumerEntryStatus {
    Unbound,
    Bound {
        handle: Rc<DestructedData>,
        source: GateProducerSocket,
    },
}

pub struct SimGateProducerEntry {
    handle: Rc<DestructedData>,

    read_only: SimData,
    write_only: Option<SimData>,

    /// consumers that depend on this producer socket
    dependents: HashSet<GateConsumerSocket>,
}

impl SimGate {
    /// get gate definition
    pub fn get_def(&self) -> &DestructedGateDefinition {
        &self.definition
    }

    /// get the data at the index-th producer of the gate
    pub fn get_producer(&self, index: usize) -> Option<&SimData> {
        Some(&self.producers.get(index)?.read_only)
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

        let mut consumers = Vec::with_capacity(definition.consumers.len());

        for entry in definition.consumers.iter() {
            match world_data.request_handle(&entry.data_type_req) {
                Some(data_type) => consumers.push(SimGateConsumerEntry {
                    request: entry.data_type_req.clone(),
                    default_data_type: data_type.clone(),
                    status: SimGateConsumerEntryStatus::Unbound,
                }),
                None => {
                    return Err(sim::Error::RequestedDataTypeNotFound {
                        data_type: entry.data_type_req.clone(),
                    }
                    .into());
                }
            }
        }

        let mut producers = Vec::with_capacity(definition.producers.len());

        for entry in definition.producers.iter() {
            match world_data.get_handle(&entry.data_type) {
                Some(data_type) => producers.push(SimGateProducerEntry {
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

            consumers,
            producers,

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

        // creates the array of pointers to consumer data
        // (is it possible to reduce the amount of cloning here?)
        let consumer_slice = slice::from_vec_rustonly(
            self.consumers
                .iter()
                .map(|consumer| match &consumer.status {
                    SimGateConsumerEntryStatus::Bound { handle, source } => {
                        match world_gates.get_producer(source) {
                            Some(data) => data.get_data_ptr(),
                            None => {
                                errors.push(sim::Error::ProducerSocketNotFound {
                                    producer_socket: *source,
                                });

                                // if producer socket not in world, treat as unbound
                                let temp_data = SimData::new_default(handle.clone());
                                let ptr = temp_data.get_data_ptr();
                                temp_datas.push(temp_data);
                                ptr
                            }
                        }
                    }
                    SimGateConsumerEntryStatus::Unbound => {
                        let temp_data = SimData::new_default(consumer.default_data_type.clone());
                        let ptr = temp_data.get_data_ptr();
                        temp_datas.push(temp_data);
                        ptr
                    }
                })
                .collect(),
        );

        let producer_slice = self.handle.tick(self.gate_ptr, &consumer_slice);

        slice::from_slice::<DataPtrMut>(&producer_slice)
            .iter()
            .zip(self.producers.iter_mut())
            .for_each(
                |(
                    &data,
                    SimGateProducerEntry {
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
        for producer in self.producers.iter_mut() {
            if let Some(new_producer) = producer.write_only.take() {
                producer.read_only = new_producer;
            }
        }
    }

    /// connect an consumer (of this gate) to an producer (of another gate).
    /// this also checks if their types are compatible
    pub fn connect_consumer_to(
        &mut self,
        consumer_socket: &GateConsumerSocket,
        producer_socket: GateProducerSocket,
        producer_type: &Rc<DestructedData>,
    ) -> Result<(), Box<sim::Error>> {
        let consumer_entry = self
            .consumers
            .get_mut(consumer_socket.get_index())
            .ok_or_else(|| {
                Box::new(sim::Error::ConsumerSocketNotFound {
                    consumer_socket: *consumer_socket,
                })
            })?;

        match consumer_entry.status {
            SimGateConsumerEntryStatus::Unbound => {
                if !consumer_entry.request.matches(producer_type.id()) {
                    return Err(sim::Error::IOTypeMismatch {
                        consumer_socket: *consumer_socket,
                        producer_socket,
                    }
                    .into());
                }

                consumer_entry.status = SimGateConsumerEntryStatus::Bound {
                    handle: producer_type.clone(),
                    source: producer_socket,
                };
                Ok(())
            }
            SimGateConsumerEntryStatus::Bound { source, .. } => {
                Err(sim::Error::ConsumerSocketDoubleBound {
                    consumer_socket: *consumer_socket,
                    current_producer: source,
                    new_producer: producer_socket,
                }
                .into())
            }
        }
    }

    /// connect an producer (of this gate) to an consumer (of another gate)
    pub fn producer_connected_from(
        &mut self,
        producer_socket: &GateProducerSocket,
        consumer_socket: GateConsumerSocket,
    ) -> Result<(), Box<sim::Error>> {
        let producer_entry = self
            .producers
            .get_mut(producer_socket.get_index())
            .ok_or_else(|| {
                Box::new(sim::Error::ProducerSocketNotFound {
                    producer_socket: *producer_socket,
                })
            })?;

        if producer_entry.dependents.insert(consumer_socket) {
            Ok(())
        } else {
            Err(sim::Error::ProducerSocketDoubleBound {
                consumer_socket,
                producer_socket: *producer_socket,
            }
            .into())
        }
    }

    /// gate data type (destructed gate) of an producer index of this gate,
    /// it does not check if the component id in the producer socket if correct,
    /// you will have to make sure that is the case yourself
    pub fn get_producer_type(
        &self,
        producer_socket: &GateProducerSocket,
    ) -> Result<&Rc<DestructedData>, Box<sim::Error>> {
        match self.producers.get(producer_socket.get_index()) {
            Some(producer_entry) => Ok(&producer_entry.handle),
            None => Err(sim::Error::ProducerSocketNotFound {
                producer_socket: *producer_socket,
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
