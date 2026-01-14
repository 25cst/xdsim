/// ID of a component in both the simulation and graphics world
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct ComponentId(u32);

impl ComponentId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// increment counter BEFORE use
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

pub struct ComponentIdIncrementer {
    content: ComponentId,
}

impl ComponentIdIncrementer {
    pub fn get(&mut self) -> ComponentId {
        self.content.increment();
        self.content
    }

    pub fn zero() -> Self {
        Self {
            content: ComponentId::new(0),
        }
    }
}
