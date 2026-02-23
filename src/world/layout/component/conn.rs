use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    common::{
        self,
        world::{
            ComponentId, ComponentIdIncrementer, ComponentIdType, GateConsumerSocket,
            GateProducerSocket, Vec2,
        },
    },
    packages::destructor::DestructedData,
    world::{layout, sim},
};

/// collection of points and segments with constraints
pub struct LayoutConn {
    points: HashMap<ComponentId, LayoutConnPoint>,
    segments: HashMap<ComponentId, LayoutConnSegment>,

    /// data type of the conn
    data_type: Rc<DestructedData>,
    /// the data producer the conn is connected to
    producer: Option<GateProducerSocket>,
    /// the data consumers the conn is connected to
    /// (number of points that are bounded to a consumer, if it drops to 0, remove the consumer)
    consumers: HashSet<GateConsumerSocket>,
}

/// returned new connection, this sturct only exist to be destructed
pub struct LayoutConnDrawRes {
    pub conn: LayoutConn,
    pub from: ComponentId,
    pub to: ComponentId,
    pub segment: ComponentId,
}

/// returned draw danging conn, this sturct only exist to be destructed
pub struct LayoutConnDrawDanglingRes {
    pub to: ComponentId,
    pub segment: ComponentId,
}

impl LayoutConn {
    /// make a new point in conn
    fn make_point(
        &mut self,
        self_id: ComponentId,
        counter: &mut ComponentIdIncrementer,
        pos: Vec2,
    ) -> ComponentId {
        let id = counter.get(ComponentIdType::ConnPoint { conn_id: self_id });

        self.points.insert(
            id,
            LayoutConnPoint {
                pos,
                before: LayoutConnPointBefore::Dangling,
                segments_after: HashSet::new(),
                consumer: None,
            },
        );

        id
    }

    /// make a segment between two points
    fn make_segment(
        &mut self,
        self_id: ComponentId,
        counter: &mut ComponentIdIncrementer,
        from: ComponentId,
        to: ComponentId,
    ) -> Result<ComponentId, Box<layout::Error>> {
        if !self.points.contains_key(&from) {
            return Err(layout::Error::ConnPointNotFound { point: from }.into());
        }

        if !self.points.contains_key(&to) {
            return Err(layout::Error::ConnPointNotFound { point: to }.into());
        }

        let id = counter.get(ComponentIdType::ConnSegment { conn_id: self_id });

        self.segments
            .insert(id, LayoutConnSegment::new_unchecked(from, to));

        Ok(id)
    }

    /// helper to get a point
    fn get_point(&self, point_id: &ComponentId) -> Result<&LayoutConnPoint, Box<layout::Error>> {
        self.points
            .get(point_id)
            .ok_or_else(|| Box::new(layout::Error::ConnPointNotFound { point: *point_id }))
    }

    /// helper to get a segment
    fn get_segment(
        &self,
        segment_id: &ComponentId,
    ) -> Result<&LayoutConnSegment, Box<layout::Error>> {
        self.segments
            .get(segment_id)
            .ok_or_else(|| Box::new(layout::Error::ConnPointNotFound { point: *segment_id }))
    }

    /// remove a conn point
    pub fn rm_point(
        &mut self,
        counter: &mut ComponentIdIncrementer,
        point_id: &ComponentId,
    ) -> Result<(), Box<layout::Error>> {
        let point = self.get_point(point_id)?;
        if point.consumer.is_some()
            || !point.before.is_dangling()
            || !point.segments_after.is_empty()
        {
            return Err(layout::Error::RmNonEmptyPoint { point: *point_id }.into());
        }

        self.points.remove(point_id);
        counter
            .unregister(point_id)
            .map_err(layout::Error::Common)?;
        Ok(())
    }

    /// bind a point to a producer
    pub fn bind_producer(
        &mut self,
        layout_gates: &mut layout::WorldStateGates,
        point_id: ComponentId,
        producer_socket: GateProducerSocket,
    ) -> Result<(), Box<layout::Error>> {
        let point = self
            .points
            .get_mut(&point_id)
            .ok_or_else(|| Box::new(layout::Error::ConnPointNotFound { point: point_id }))?;

        if self.producer.is_some() {
            return Err(layout::Error::ConnPointDoubleBindProducer { point: point_id }.into());
        }

        layout_gates.point_bind_producer(&producer_socket, point_id)?;

        point.before = LayoutConnPointBefore::Producer { producer_socket };
        self.producer = Some(producer_socket);
        Ok(())
    }

    /// bind a point to a consumer
    pub fn bind_consumer(
        &mut self,
        sim_world: &mut sim::WorldState,
        layout_gates: &mut layout::WorldStateGates,
        point_id: ComponentId,
        consumer_socket: GateConsumerSocket,
    ) -> Result<(), Box<layout::Error>> {
        let point = self
            .points
            .get_mut(&point_id)
            .ok_or_else(|| Box::new(layout::Error::ConnPointNotFound { point: point_id }))?;

        if let Some(producer) = self.producer {
            sim_world
                .connect_gates(sim::requests::ConnectIOSockets {
                    consumer_socket,
                    producer_socket: producer,
                })
                .map_err(layout::Error::Sim)?;
        }

        layout_gates.point_bind_consumer(&consumer_socket, point_id)?;

        point.consumer = Some(consumer_socket);
        self.consumers.insert(consumer_socket);
        Ok(())
    }

    /// draw a new segment from a producer socket
    pub fn draw_new(
        self_id: ComponentId,
        counter: &mut ComponentIdIncrementer,
        sim_world: &sim::WorldState,
        layout_gates: &mut layout::WorldStateGates,
        from: GateProducerSocket,
        to: Vec2,
    ) -> Result<LayoutConnDrawRes, Box<layout::Error>> {
        let sim_gate = sim_world
            .get_gate(from.get_id())
            .map_err(layout::Error::Sim)?;

        let data_type = sim_gate
            .get_producer_type(&from)
            .map_err(layout::Error::Sim)?
            .clone();

        let layout_gate = layout_gates.get_gate(from.get_id())?;

        let mut out = Self {
            points: HashMap::new(),
            segments: HashMap::new(),
            data_type,
            producer: None,
            consumers: HashSet::new(),
        };

        let from_id = out.make_point(
            self_id,
            counter,
            layout_gate.get_pos()
                + layout_gate
                    .get_producer_rel_pos(&from)?
                    .rotate(layout_gate.get_rotation()),
        );

        out.bind_producer(layout_gates, from_id, from)
            .inspect_err(|_| {
                let _ = out.rm_point(counter, &from_id);
            })?;

        let to_id = out.make_point(self_id, counter, to);
        let segment_id = out
            .make_segment(self_id, counter, from_id, to_id)
            .inspect_err(|_| {
                let _ = out.rm_point(counter, &from_id);
                let _ = out.rm_point(counter, &to_id);
            })?;

        Ok(LayoutConnDrawRes {
            conn: out,
            from: from_id,
            to: to_id,
            segment: segment_id,
        })
    }

    /// draw a new segment from a point in this conn
    ///
    /// returns the component ID of the new dangling point
    pub fn draw_dangling(
        &mut self,
        self_id: ComponentId,
        counter: &mut ComponentIdIncrementer,
        from: ComponentId,
        to: Vec2,
    ) -> Result<LayoutConnDrawDanglingRes, Box<layout::Error>> {
        if !self.points.contains_key(&from) {
            return Err(layout::Error::ConnPointNotFound { point: from }.into());
        }

        let to_id = self.make_point(self_id, counter, to);
        let segment_id = self
            .make_segment(self_id, counter, from, to_id)
            .inspect_err(|_| {
                let _ = self.rm_point(counter, &to_id);
            })?;

        Ok(LayoutConnDrawDanglingRes {
            to: to_id,
            segment: segment_id,
        })
    }
}

impl LayoutConn {
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

/// a point
///
/// each point can only have at most one consumer (includes producer and segments before)
struct LayoutConnPoint {
    /// position of the layout on canvas
    pos: Vec2,
    /// the connections/producer socket before this point
    before: LayoutConnPointBefore,
    /// the segments connected after the point
    segments_after: HashSet<ComponentId>,
    /// the consumer this point outputs to
    consumer: Option<GateConsumerSocket>,
}

/// stuff that happens before the point
enum LayoutConnPointBefore {
    /// one producer socket
    Producer { producer_socket: GateProducerSocket },
    /// one segment
    Segment { segment_id: ComponentId },
    /// nothing
    Dangling,
}

impl LayoutConnPointBefore {
    pub fn is_dangling(&self) -> bool {
        matches!(self, Self::Dangling)
    }
}

/// a segment connecting between two points,
/// the from and to component IDs are points
/// that exists inside the conn
struct LayoutConnSegment {
    from: ComponentId,
    to: ComponentId,
}

impl LayoutConnSegment {
    pub fn new_unchecked(from: ComponentId, to: ComponentId) -> Self {
        Self { from, to }
    }
}
