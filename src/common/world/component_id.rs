use std::collections::HashMap;

use crate::common;

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

/// type the component ID points to
pub enum ComponentIdType {
    Gate,
    Conn,
    ConnPoint { conn_id: ComponentId },
    ConnSegment { conn_id: ComponentId },
}

/// each world has a shared counter to ensure all component IDs are unique
pub struct ComponentIdIncrementer {
    content: ComponentId,
    id_types: HashMap<ComponentId, ComponentIdType>,
}

impl ComponentIdIncrementer {
    /// get a unique ID
    pub fn get(&mut self, component_type: ComponentIdType) -> ComponentId {
        self.content.increment();
        self.id_types.insert(self.content, component_type);
        self.content
    }

    /// unregister an ID when the component is removed
    pub fn unregister(&mut self, component_id: &ComponentId) -> Result<(), Box<common::Error>> {
        if self.id_types.remove(component_id).is_none() {
            Err(common::Error::UnregisterNonexistentComponentId { id: *component_id }.into())
        } else {
            Ok(())
        }
    }

    /// get the zero-ed incrementer
    pub fn zero() -> Self {
        Self {
            content: ComponentId::new(0),
            id_types: HashMap::new(),
        }
    }
}
