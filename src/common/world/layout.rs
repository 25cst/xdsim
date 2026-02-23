use std::{
    f64::consts::PI,
    ops::{Add, AddAssign},
};

#[derive(Clone, Copy)]
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

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn rotate(&self, rotation: Rotation) -> Self {
        Self {
            x: self.x * rotation.rad().cos() - self.y * rotation.rad().sin(),
            y: self.x * rotation.rad().sin() + self.y * rotation.rad().cos(),
        }
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
#[derive(Clone, Copy)]
pub struct Rotation(f64);

impl Rotation {
    /// normalise it to between 0 and 2pi
    fn normalise(&mut self) {
        self.0 = self.0.rem_euclid(2.0 * PI);
    }

    pub const fn zero() -> Self {
        Rotation(0.0)
    }

    pub fn new(angle: f64) -> Self {
        let mut out = Self(angle);
        out.normalise();
        out
    }

    pub fn rad(&self) -> f64 {
        self.0
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

pub struct BoundingBox {
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

impl From<xdsim_cbinds::common::BoundingBox> for BoundingBox {
    fn from(value: xdsim_cbinds::common::BoundingBox) -> Self {
        Self {
            top: value.top,
            bottom: value.bottom,
            left: value.left,
            right: value.right,
        }
    }
}
