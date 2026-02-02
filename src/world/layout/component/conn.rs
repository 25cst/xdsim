use std::{
    cell::UnsafeCell,
    collections::{HashMap, HashSet},
};

use crate::{
    common::world::{
        ComponentId, ComponentIdIncrementer, Direction, GateInputSocket, GateOutputSocket, Vec2,
    },
    world::layout::{self, LayoutSegment},
};

/// Connection paths and position
pub struct LayoutConn {
    /// origin position
    position: Vec2,
    /// origin segment: the ID for the first segment,
    /// this segment is guaranteed to exist in segments
    ///
    /// Note: if the Conn has no segments, it is removed
    origin: ComponentId,
    /// the producer (output socket) that the conn is connected to
    producer: Option<GateInputSocket>,
    /// the consumers (input sockets) that the conn is connected to
    consumers: HashSet<GateOutputSocket>,
    /// the actual segments that make up the conn
    segments: HashMap<ComponentId, UnsafeCell<LayoutSegment>>,
}

impl LayoutConn {
    /// get a segment
    pub fn get(&self, segment_id: &ComponentId) -> Result<&LayoutSegment, Box<layout::Error>> {
        unsafe { self.get_mut_unsafe(segment_id).map(|c| &*c) }
    }

    /// get a segment (mutable)
    pub fn get_mut(
        &mut self,
        segment_id: &ComponentId,
    ) -> Result<&mut LayoutSegment, Box<layout::Error>> {
        unsafe { self.get_mut_unsafe(segment_id) }
    }

    /// DANGER: does not guarantee reference valid after ANY operation,
    /// it is only guaranteed to be valid immediately after getting
    ///
    /// get a segment (mutable, unsafe)
    unsafe fn get_mut_unsafe(
        &self,
        segment_id: &ComponentId,
    ) -> Result<&'static mut LayoutSegment, Box<layout::Error>> {
        match self.segments.get(segment_id) {
            Some(segment) => Ok(unsafe { &mut *segment.get() }),
            None => Err(layout::Error::SegmentNotFound {
                segment_id: *segment_id,
            }
            .into()),
        }
    }

    /// insert a new segment with an ID without performing any checks
    pub fn insert_unchecked(&mut self, id: ComponentId, segment: LayoutSegment) {
        self.segments.insert(id, UnsafeCell::new(segment));
    }

    /// add a new segment after an existing segment
    pub fn add_segment_after(
        &mut self,
        id_counter: &mut ComponentIdIncrementer,
        after_segment: ComponentId,
        direction: Direction,
        length: f64,
    ) -> Result<ComponentId, Box<layout::Error>> {
        unsafe {
            self.get_mut_unsafe(&after_segment)?.add_segment_back(
                id_counter,
                self,
                after_segment,
                direction,
                length,
            )
        }
    }

    /// set length of a segment with a dangling end
    pub fn set_length_end_dangling(
        &mut self,
        segment_id: &ComponentId,
        length: f64,
    ) -> Result<(), Box<layout::Error>> {
        self.get_mut(segment_id)?
            .set_length_end_dangling(segment_id, length)
    }

    /// set length of a segment with a dangling start
    pub fn set_length_start_dangling(
        &mut self,
        segment_id: &ComponentId,
        length: f64,
    ) -> Result<(), Box<layout::Error>> {
        self.get_mut(segment_id)?
            .set_length_start_dangling(segment_id, length)
    }
}
