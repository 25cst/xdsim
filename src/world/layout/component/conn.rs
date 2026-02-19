use std::collections::{HashMap, HashSet};

use crate::{
    common::world::{
        ComponentId, ComponentIdIncrementer, ComponentIdType, GateConsumerSocket,
        GateProducerSocket, Vec2,
    },
    world::layout,
};

/// collection of points and segments with constraints
pub struct LayoutConn {
    points: HashMap<ComponentId, LayoutConnPoint>,
    segments: HashMap<ComponentId, LayoutConnSegment>,
    /// the data producer the conn is connected to
    producer: Option<GateProducerSocket>,
    /// the data consumers the conn is connected to
    consumers: HashMap<GateConsumerSocket, u64>,
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
