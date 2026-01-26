use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use semver::Version;

use crate::packages::destructor::DestructedConn;

pub type DestructedConnHandles =
    HashMap<PackageName, BTreeMap<PackageVersion, HashMap<ComponentName, Rc<DestructedConn>>>>;

pub type PackageName = String;
pub type PackageVersion = Version;
pub type ComponentName = String;
