use std::ops::{Add, AddAssign};

#[derive(Clone, Copy)]
pub struct Vec2 {
    x: i64,
    y: i64,
}

impl From<xdsim_cbinds::common::Vec2> for Vec2 {
    fn from(value: xdsim_cbinds::common::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Vec2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn new_with_direction(direction: Direction, length: i64) -> Self {
        match direction {
            Direction::Up => Self::new(0, length),
            Direction::Right => Self::new(length, 0),
            Direction::Down => Self::new(0, -length),
            Direction::Left => Self::new(-length, 0),
        }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        self.rotate(Rotation::D180)
    }
}

impl From<xdsim_cbinds::common::Direction> for Direction {
    fn from(value: xdsim_cbinds::common::Direction) -> Self {
        match value {
            xdsim_cbinds::common::Direction::Up => Self::Up,
            xdsim_cbinds::common::Direction::Right => Self::Right,
            xdsim_cbinds::common::Direction::Down => Self::Down,
            xdsim_cbinds::common::Direction::Left => Self::Left,
        }
    }
}

/// rotation to apply to direction
#[derive(Clone, Copy)]
pub enum Rotation {
    D0,
    D90,
    D180,
    D270,
}

impl Direction {
    /// apply rotation to direction
    pub fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::D0 => *self,
            Rotation::D90 => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            Rotation::D180 => match self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            },
            Rotation::D270 => match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
        }
    }
}
