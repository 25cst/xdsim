use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use semver::Version;

use crate::{
    common::world::{ComponentVersion, Vec2},
    packages::destructor::{DestructedConn, DestructedData, DestructedGate},
};

pub type DestructedConnHandles =
    HashMap<PackageName, BTreeMap<PackageVersion, HashMap<ComponentName, Rc<DestructedConn>>>>;
pub type DestructedGateHandles =
    HashMap<PackageName, BTreeMap<PackageVersion, HashMap<ComponentName, Rc<DestructedGate>>>>;
pub type DestructedDataHandles =
    HashMap<PackageName, BTreeMap<PackageVersion, HashMap<ComponentName, Rc<DestructedData>>>>;

pub type PackageName = String;
pub type PackageVersion = Version;
pub type ComponentName = String;

/// WorldState::create_default_gate(CreateDefaultGate)
pub struct CreateDefaultGate {
    /// Identifier of the gate type
    pub gate: ComponentVersion,
    /// bottom left corner (origin) of the gate
    pub origin: Vec2,
}

/// `WorldState::new_blank(CreateBlankWorld) -> WorldState`
pub struct CreateBlankWorld {
    /// All the data that can be used in the world
    pub data_handles: DestructedDataHandles,
    /// All the gates that can be used in the world
    pub gate_handles: DestructedGateHandles,
    /// All the conn that can be used in the world
    pub conn_handles: DestructedConnHandles,
}
