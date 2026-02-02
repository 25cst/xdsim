/// ID of a component in both the simulation and graphics world
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct ComponentId(u64);

impl ComponentId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// increment counter BEFORE use
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

/// each world has a shared counter to ensure all component IDs are unique
pub struct ComponentIdIncrementer {
    content: ComponentId,
}

impl ComponentIdIncrementer {
    /// get a unique ID
    pub fn get(&mut self) -> ComponentId {
        self.content.increment();
        self.content
    }

    /// get the zero-ed incrementer
    pub fn zero() -> Self {
        Self {
            content: ComponentId::new(0),
        }
    }
}
