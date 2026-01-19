//! Requests to poke the world state to do stuff.
use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use semver::Version;

use crate::{
    common::world::{ComponentVersion, GateInputSocket, GateOutputSocket},
    packages::destructor::{DestructedData, DestructedGate},
};

pub type DestructedGateHandles =
    HashMap<PackageName, BTreeMap<PackageVersion, HashMap<ComponentName, Rc<DestructedGate>>>>;
pub type DestructedDataHandles =
    HashMap<PackageName, BTreeMap<PackageVersion, HashMap<ComponentName, Rc<DestructedData>>>>;

pub type PackageName = String;
pub type PackageVersion = Version;
pub type ComponentName = String;

/// `WorldState::new_blank(CreateBlankWorld) -> WorldState`
pub struct CreateBlankWorld {
    /// All the data that can be used in the world
    pub data_handles: DestructedDataHandles,
    /// All the gates that can be used in the world
    pub gate_handles: DestructedGateHandles,
}

impl CreateBlankWorld {
    /// Create a world state with no gates
    pub fn empty() -> Self {
        Self {
            data_handles: HashMap::new(),
            gate_handles: HashMap::new(),
        }
    }
}

/// `WorldState::create_default_gate(CreateDefaultGate) -> Result&lt;ComponentId&gt;`
pub struct CreateDefaultGate {
    /// Identifier of the gate type
    pub gate: ComponentVersion,
}

/// `WorldState::connect_gates(ConnectIOSockets)  -> Result&lt;()&gt;`
pub struct ConnectIOSockets {
    // self explanatory
    pub input_socket: GateInputSocket,
    pub output_socket: GateOutputSocket,
}
