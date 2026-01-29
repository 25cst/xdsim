pub struct Vec2 {
    x: f64,
    y: f64,
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
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &f64 {
        &self.x
    }

    pub fn y(&self) -> &f64 {
        &self.y
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
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
    pub fn apply(&self, rotation: Rotation) -> Self {
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
