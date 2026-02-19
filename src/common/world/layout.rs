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

/// angle counter clockwise from the x-axis
pub struct Rotation(f64);

impl Rotation {
    /// normalise it to between 0 and 360
    fn normalise(&mut self) {
        self.0 = self.0.rem_euclid(360.0);
    }

    pub const fn zero() -> Self {
        Rotation(0.0)
    }

    pub fn new(angle: f64) -> Self {
        let mut out = Self(angle);
        out.normalise();
        out
    }
}

impl Add for Rotation {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0 += rhs.0;
        self.normalise();
        self
    }
}

impl AddAssign for Rotation {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.normalise();
    }
}
