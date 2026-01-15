//! Requests to poke the world state to do stuff.
use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use semver::Version;

use crate::{
    common::world::{ComponentId, ComponentVersion, GateInputSocket, GateOutputSocket},
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

/// `WorldState::register_new_gate_output(RegisterNewGateOutput) -> Result&lt;ComponentId&gt;`
pub struct RegisterNewGateOutput {
    /// Socket of gate that will be connected to the buffer
    pub socket: GateOutputSocket,
}

/// `WorldState::register_existing_gate_output(RegisterExistingGateOutput) -> Result&lt;()&gt;`
pub struct RegisterExistingGateOutput {
    /// Socket of gate that will be connected to the buffer
    pub socket: GateOutputSocket,
    /// Id of the buffer to put outputs to
    pub buffer: ComponentId,
}

/// `WorldState::register_existing_gate_input(RegisterExistingGateInput) -> Result&lt;()&gt;`
pub struct RegisterExistingGateInput {
    /// Socket of gate that will be connected to the buffer
    pub socket: GateInputSocket,
    /// Id of the buffer to take inputs to
    pub buffer: ComponentId,
}
