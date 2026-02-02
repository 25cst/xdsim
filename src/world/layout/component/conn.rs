use std::{
    cell::UnsafeCell,
    collections::{BTreeSet, HashMap, HashSet},
    mem,
};

use crate::{
    common::world::{
        ComponentId, ComponentIdIncrementer, Direction, GateInputSocket, GateOutputSocket, Vec2,
    },
    world::layout,
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
    fn set_length_end_dangling(
        &mut self,
        segment_id: &ComponentId,
        length: f64,
    ) -> Result<(), Box<layout::Error>> {
        self.get_mut(segment_id)?
            .set_length_end_dangling(segment_id, length)
    }

    /// set length of a segment with a dangling start
    fn set_length_start_dangling(
        &mut self,
        segment_id: &ComponentId,
        length: f64,
    ) -> Result<(), Box<layout::Error>> {
        self.get_mut(segment_id)?
            .set_length_start_dangling(segment_id, length)
    }
}

/// a single segment in a conn
pub struct LayoutSegment {
    /// relative position to the origin of the conn
    position: Vec2,
    /// the direction the end point is to the starting point
    direction: Direction,
    /// the previous thing that the segment is connected to
    previous: LayoutSegmentPrevious,
    /// the next thing that the segment is connected to
    next: LayoutSegmentNext,
    /// length of the segment
    length: f64,
}

impl LayoutSegment {
    /// # Safety
    ///
    /// the reference to Self will become invalid after running this
    pub unsafe fn add_segment_back(
        &mut self,
        id_counter: &mut ComponentIdIncrementer,
        conns: &mut LayoutConn,
        self_segment_id: ComponentId,
        direction: Direction,
        length: f64,
    ) -> Result<ComponentId, Box<layout::Error>> {
        let next_segments = match &mut self.next {
            LayoutSegmentNext::InputSocket(_) => {
                return Err(layout::Error::NewSegmentOnSocket {
                    segment_id: self_segment_id,
                }
                .into());
            }
            LayoutSegmentNext::Segments(segments) => {
                if segments.is_empty() && direction == self.direction {
                    return Err(layout::Error::NewSegmentDirectionConflict {
                        segment_id: self_segment_id,
                        direction,
                    }
                    .into());
                }

                for segment_id in segments.iter() {
                    if conns.get(segment_id)?.direction == direction {
                        return Err(layout::Error::NewSegmentDirectionConflict {
                            segment_id: self_segment_id,
                            direction,
                        }
                        .into());
                    }
                }
                segments
            }
        };

        let new_id = id_counter.get();
        next_segments.insert(new_id);

        // after inserting, the self reference may become invalid
        conns.insert_unchecked(
            new_id,
            Self {
                position: Vec2::new_with_direction(direction, length) + self.position,
                previous: LayoutSegmentPrevious::Segment(self_segment_id),
                direction,
                length,
                next: LayoutSegmentNext::Segments(BTreeSet::new()),
            },
        );

        Ok(new_id)
    }

    /// # Safety
    ///
    /// The reference to self may be invalid after this function
    unsafe fn create_new_junction_unchecked(
        &mut self,
        id_counter: &mut ComponentIdIncrementer,
        conns: &mut LayoutConn,
        self_segment_id: ComponentId,
        at_length: f64,
    ) {
        let new_id = id_counter.get();

        let mut new_next = LayoutSegmentNext::Segments(BTreeSet::from([new_id]));
        mem::swap(&mut new_next, &mut self.next);

        let next = Self {
            position: self.position + Vec2::new_with_direction(self.direction, at_length),
            direction: self.direction,
            previous: LayoutSegmentPrevious::Segment(self_segment_id),
            next: new_next,
            length: self.length - at_length,
        };

        self.length = at_length;
        self.next = LayoutSegmentNext::Segments(BTreeSet::from([new_id]));

        conns.insert_unchecked(new_id, next);
    }

    /// set length of a segment with a dangling end
    fn set_length_end_dangling(
        &mut self,
        self_segment_id: &ComponentId,
        length: f64,
    ) -> Result<(), Box<layout::Error>> {
        match &self.next {
            LayoutSegmentNext::Segments(segments) if segments.is_empty() => {}
            _ => {
                return Err(layout::Error::SegmentNotDangling {
                    segment_id: *self_segment_id,
                }
                .into());
            }
        }

        self.length = length;
        Ok(())
    }

    /// set length of a segment with a dangling start
    fn set_length_start_dangling(
        &mut self,
        self_segment_id: &ComponentId,
        length: f64,
    ) -> Result<(), Box<layout::Error>> {
        if !matches!(self.previous, LayoutSegmentPrevious::Dangling) {
            return Err(layout::Error::SegmentNotDangling {
                segment_id: *self_segment_id,
            }
            .into());
        }

        self.position += Vec2::new_with_direction(self.direction, self.length - length);
        self.length = length;
        Ok(())
    }
}

pub enum LayoutSegmentPrevious {
    /// another segment
    ///
    /// this segment must exist in LayoutConn::segments
    Segment(ComponentId),
    /// an output socket
    OutputSocket(GateOutputSocket),
    /// literally nothing
    Dangling,
}

#[derive(Hash, PartialEq, Eq)]
pub enum LayoutSegmentNext {
    /// the segments that are connected to the end of this segment
    ///
    /// this segment must exist in LayoutConn::segments
    Segments(BTreeSet<ComponentId>),
    /// an input socket
    InputSocket(GateInputSocket),
}
