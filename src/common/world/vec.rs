pub struct Vec2 {
    x: f32,
    y: f32,
}

impl From<&xdsim_cbinds::common::Vec2> for Vec2 {
    fn from(value: &xdsim_cbinds::common::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}
