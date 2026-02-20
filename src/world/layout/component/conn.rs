use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    common::world::{
        ComponentId, ComponentIdIncrementer, ComponentIdType, GateConsumerSocket,
        GateProducerSocket, Vec2,
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
    consumers: HashMap<GateConsumerSocket, u64>,
}

/// returned new connection, this sturct only exist to be destructed
pub struct LayoutConnDrawNewRes {
    pub conn: LayoutConn,
    pub segment_id: ComponentId,
    pub dangling_point: ComponentId,
}

/// returned new segment and point in connection, this sturct only exist to be destructed
pub struct LayoutConnDrawDanglingRes {
    pub segment_id: ComponentId,
    pub dangling_point: ComponentId,
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

    /// bind a point to a producer
    pub fn bind_producer(
        &mut self,
        point_id: &ComponentId,
        producer_socket: GateProducerSocket,
    ) -> Result<(), Box<layout::Error>> {
        let point = self
            .points
            .get_mut(point_id)
            .ok_or_else(|| Box::new(layout::Error::ConnPointNotFound { point: *point_id }))?;

        if self.producer.is_some() {
            return Err(layout::Error::ConnPointDoubleBindProducer { point: *point_id }.into());
        }

        point.before = LayoutConnPointBefore::Producer {
            producer_socket: producer_socket,
        };
        self.producer = Some(producer_socket);
        Ok(())
    }

    /// bind a point to a consumer
    pub fn bind_consumer(
        &mut self,
        point_id: &ComponentId,
        consumer_socket: GateConsumerSocket,
    ) -> Result<(), Box<layout::Error>> {
        let point = self
            .points
            .get_mut(point_id)
            .ok_or_else(|| Box::new(layout::Error::ConnPointNotFound { point: *point_id }))?;

        point.consumer = Some(consumer_socket);
        *self.consumers.entry(consumer_socket).or_default() += 1;
        Ok(())
    }

    /// draw a new segment from a producer socket
    pub fn draw_new(
        self_id: ComponentId,
        counter: &mut ComponentIdIncrementer,
        sim_world: &sim::WorldState,
        layout_gates: &layout::WorldStateGates,
        from: GateProducerSocket,
        to: Vec2,
    ) -> Result<LayoutConnDrawNewRes, Box<layout::Error>> {
        let sim_gate = sim_world
            .get_gate(from.get_id())
            .map_err(layout::Error::from_sim)?;

        let def = sim_gate.get_def();
        let data_type = sim_gate
            .get_producer_type(&from)
            .map_err(layout::Error::from_sim)?
            .clone();

        let layout_gate = layout_gates.get_gate(from.get_id())?;

        let mut out = Self {
            points: HashMap::new(),
            segments: HashMap::new(),
            data_type,
            producer: None,
            consumers: HashMap::new(),
        };

        let from_id = out.make_point(
            self_id,
            counter,
            layout_gate.get_pos()
                + Vec2::from(def.consumers[from.get_index()].position)
                    .rotate(layout_gate.get_rotation()),
        );
        let to_id = out.make_point(self_id, counter, to);
        let segment_id = out.make_segment(self_id, counter, from_id, to_id)?;

        Ok(LayoutConnDrawNewRes {
            conn: out,
            segment_id,
            dangling_point: to_id,
        })
    }

    /// draw a new segment from a point in this conn
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
        let segment_id = self.make_segment(self_id, counter, from, to_id)?;

        Ok(LayoutConnDrawDanglingRes {
            segment_id,
            dangling_point: to_id,
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
