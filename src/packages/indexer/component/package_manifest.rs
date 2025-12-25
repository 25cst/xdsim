use std::collections::HashMap;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

/// package.toml manifest
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "devel", derive(Debug))]
pub struct PackageManifest {
    package: PackageInfo,
    dependencies: HashMap<String, VersionReq>,
    provides: HashMap<String, PackageComponentType>,
}

impl PackageManifest {
    pub fn get_name(&self) -> &str {
        &self.package.name
    }

    pub fn get_version(&self) -> &Version {
        &self.package.version
    }
}

/// has public fields,
/// but the struct is private
/// so it is fine.
/// the struct exists solely to add structure to the toml file
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "devel", derive(Debug))]
struct PackageInfo {
    pub name: String,
    pub version: Version,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "devel", derive(Debug))]
pub enum PackageComponentType {
    #[serde(rename = "gate")]
    Gate,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "connection")]
    Connection,
}

impl PackageManifest {}
