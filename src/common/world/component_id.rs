use std::collections::HashMap;

use crate::common;

/// ID of a component in both the simulation and graphics world
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct ComponentId(u64);

impl ComponentId {
    fn new(id: u64) -> Self {
        Self(id)
    }

    /// increment counter BEFORE use
    fn increment(&mut self) {
        self.0 += 1;
    }
}

/// type the component ID points to
#[derive(Clone, Copy)]
pub enum ComponentIdType {
    Gate,
    Conn,
    ConnPoint { conn_id: ComponentId },
    ConnSegment { conn_id: ComponentId },
}

#[derive(Clone, Copy)]
pub enum ComponentIdTypeName {
    Gate,
    Conn,
    ConnPoint,
    ConnSegment,
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

    /// get type of thing the ID is
    pub fn get_type(&self, id: &ComponentId) -> Result<ComponentIdType, Box<common::Error>> {
        self.id_types
            .get(id)
            .copied()
            .ok_or_else(|| Box::new(common::Error::ComponentIdLookupNotFound { id: *id }))
    }

    /// assert that the type is a gate
    pub fn assert_gate(&self, id: &ComponentId) -> Result<(), Box<common::Error>> {
        match self.get_type(id)? {
            ComponentIdType::Gate => Ok(()),
            other => Err(common::Error::ComponentIdTypeMismatch {
                id: *id,
                expected: ComponentIdTypeName::Gate,
                got: other,
            }
            .into()),
        }
    }

    /// assert that the type is a conn
    pub fn assert_conn(&self, id: &ComponentId) -> Result<(), Box<common::Error>> {
        match self.get_type(id)? {
            ComponentIdType::Conn => Ok(()),
            other => Err(common::Error::ComponentIdTypeMismatch {
                id: *id,
                expected: ComponentIdTypeName::Conn,
                got: other,
            }
            .into()),
        }
    }

    /// assert that the type is a conn point, return the ID of the conn containing the point
    pub fn assert_conn_point(&self, id: &ComponentId) -> Result<ComponentId, Box<common::Error>> {
        match self.get_type(id)? {
            ComponentIdType::ConnPoint { conn_id } => Ok(conn_id),
            other => Err(common::Error::ComponentIdTypeMismatch {
                id: *id,
                expected: ComponentIdTypeName::ConnPoint,
                got: other,
            }
            .into()),
        }
    }

    /// assert that the type is a conn segment, return the ID of the conn containing the point
    pub fn assert_conn_segment(&self, id: &ComponentId) -> Result<ComponentId, Box<common::Error>> {
        match self.get_type(id)? {
            ComponentIdType::ConnSegment { conn_id } => Ok(conn_id),
            other => Err(common::Error::ComponentIdTypeMismatch {
                id: *id,
                expected: ComponentIdTypeName::ConnSegment,
                got: other,
            }
            .into()),
        }
    }
}
