use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use semver::Version;

pub type DestructedConnHandles =
    HashMap<PackageName, BTreeMap<PackageVersion, HashMap<ComponentName, Rc<DestructedGate>>>>;

pub type PackageName = String;
pub type PackageVersion = Version;
pub type ComponentName = String;
