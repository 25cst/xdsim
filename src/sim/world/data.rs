use std::{
    collections::{HashMap, HashSet},
    mem,
    rc::Rc,
};

use crate::{
    common::world::{
        ComponentId, ComponentIdIncrementer, ComponentVersion, ComponentVersionReq,
        GateInputSocket, GateOutputSocket,
    },
    packages::destructor::DestructedData,
    sim::{self, component::SimData, world::*},
};

pub type DestructedDataHandles =
    HashMap<PackageName, HashMap<PackageVersion, HashMap<ComponentName, Rc<DestructedData>>>>;

struct RegisteredBuffer {
    data_type: Rc<DestructedData>,
    /// where the data comes from
    producer: Option<GateOutputSocket>,
    /// where the data goes
    consumers: HashSet<GateInputSocket>,

    read_only: SimData,
    write_only: Option<SimData>,
}

impl RegisteredBuffer {
    pub fn read(&self) -> &SimData {
        &self.read_only
    }

    /// write to a the buffer if it hasn't yet been written to this tick
    pub fn write(&mut self, self_id: &ComponentId, data: SimData) -> Result<(), Box<sim::Error>> {
        if self.write_only.is_some() {
            return Err(sim::Error::BufferDoubleWrite {
                buffer_id: *self_id,
            }
            .into());
        }

        self.write_only = Some(data);
        Ok(())
    }

    /// flush updates
    pub fn flush(&mut self) {
        if let Some(data) = self.write_only.take() {
            self.read_only = data;
        }
    }

    pub fn new(data_type: Rc<DestructedData>) -> Self {
        Self {
            data_type: data_type.clone(),
            producer: None,
            consumers: HashSet::new(),
            read_only: SimData::new_default(data_type),
            write_only: None,
        }
    }

    pub fn new_with_producer(
        data_type: Rc<DestructedData>,
        producer_socket: GateOutputSocket,
    ) -> Self {
        Self {
            data_type: data_type.clone(),
            producer: Some(producer_socket),
            consumers: HashSet::new(),
            read_only: SimData::new_default(data_type),
            write_only: None,
        }
    }

    /// set the gate socket that outputs to this buffer
    pub fn set_producer(
        &mut self,
        self_id: &ComponentId,
        gate_socket: GateOutputSocket,
    ) -> Result<(), Box<sim::Error>> {
        if self.producer.is_some() {
            return Err(sim::Error::BufferDoubleProducerRegister {
                gate_type: self.data_type.id().clone(),
                gate_socket,
                buffer_id: *self_id,
            }
            .into());
        }

        self.producer = Some(gate_socket);
        Ok(())
    }

    pub fn remove_producer(
        &mut self,
        self_id: &ComponentId,
    ) -> Result<GateOutputSocket, Box<sim::Error>> {
        match self.producer.take() {
            Some(producer) => Ok(producer),
            None => Err(sim::Error::BufferNoProducerToRemove {
                buffer_id: *self_id,
            }
            .into()),
        }
    }
}

pub struct WorldStateData {
    /// all data types
    // may one day replace the Rc in SimData with a dumb pointer because it is guaranteed to exist
    // as owned here
    handles: DestructedDataHandles,

    /// The data type of each buffer
    buffers: HashMap<ComponentId, RegisteredBuffer>,
}

impl WorldStateData {
    /// create world state data with only handles and no buffers in world
    pub fn new_blank(handles: DestructedDataHandles) -> Self {
        Self {
            handles,
            buffers: HashMap::new(),
        }
    }

    /// Get handle using a ComponentVersion
    pub fn get_handle(&self, component: &ComponentVersion) -> Option<&Rc<DestructedData>> {
        self.handles
            .get(&component.package)?
            .get(&component.version)?
            .get(&component.component)
    }

    /// Get handle using a ComponentVersionReq
    pub fn request_handle(
        &self,
        component_req: &ComponentVersionReq,
    ) -> Option<&Rc<DestructedData>> {
        self.handles
            .get(&component_req.package)?
            .iter()
            .find(|(version, _)| component_req.version_req.matches(version))?
            .1
            .get(&component_req.component)
    }

    /// read from current world state
    pub fn read_buffer(&self, buf_id: &ComponentId) -> Option<&SimData> {
        self.buffers.get(buf_id).map(RegisteredBuffer::read)
    }

    /// write to next tick's world state
    pub fn write_buffer(
        &mut self,
        buffer_id: &ComponentId,
        content: SimData,
    ) -> Result<(), Box<sim::Error>> {
        match self.buffers.get_mut(buffer_id) {
            Some(buffer) => {
                buffer.write(buffer_id, content)?;
                Ok(())
            }
            None => Err(sim::Error::BufferNotFound {
                buffer_id: *buffer_id,
            }
            .into()),
        }
    }

    /// end the current tick and write all updates in the current tick to world state
    pub fn flush(&mut self) {
        self.buffers.values_mut().for_each(RegisteredBuffer::flush);
    }

    /// create a new buffer with specified data type
    pub fn register_new_buffer(
        &mut self,
        data_type: Rc<DestructedData>,
        id_counter: &mut ComponentIdIncrementer,
    ) -> ComponentId {
        let buffer_id = id_counter.get();

        self.buffers
            .insert(buffer_id, RegisteredBuffer::new(data_type));
        buffer_id
    }

    /// create a new buffer with specified data type
    pub fn register_new_buffer_with_producer(
        &mut self,
        data_type: Rc<DestructedData>,
        id_counter: &mut ComponentIdIncrementer,
        producer_socket: GateOutputSocket,
    ) -> ComponentId {
        let buffer_id = id_counter.get();

        self.buffers.insert(
            buffer_id,
            RegisteredBuffer::new_with_producer(data_type, producer_socket),
        );
        buffer_id
    }

    /// set the producer (gate socket producing the output) for a buffer
    pub fn set_buffer_producer(
        &mut self,
        buffer_id: &ComponentId,
        producer_socket: GateOutputSocket,
        data_type_assert: &ComponentVersion,
    ) -> Result<(), Box<sim::Error>> {
        match self.buffers.get_mut(buffer_id) {
            Some(buffer) => {
                if buffer.data_type.id() != data_type_assert {
                    return Err(sim::Error::BufferTypeMismatch {
                        buffer_id: *buffer_id,
                        expected_type: buffer.data_type.id().clone(),
                        got_type: data_type_assert.clone(),
                    }
                    .into());
                }

                buffer.set_producer(buffer_id, producer_socket)
            }
            None => Err(sim::Error::BufferNotFound {
                buffer_id: *buffer_id,
            }
            .into()),
        }
    }

    /// remove the existing producer of a buffer
    pub fn remove_buffer_producer(
        &mut self,
        buffer_id: &ComponentId,
    ) -> Result<GateOutputSocket, Box<sim::Error>> {
        let buffer = match self.buffers.get_mut(buffer_id) {
            Some(buffer) => buffer,
            None => {
                return Err(sim::Error::BufferNotFound {
                    buffer_id: *buffer_id,
                }
                .into());
            }
        };

        buffer.remove_producer(buffer_id)
    }
}
