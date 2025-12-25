use std::collections::HashMap;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct PackageManifest {
    name: String,
    version: Version,
    require: HashMap<String, VersionReq>,
    provides: HashMap<String, PackageComponentType>,
}

impl PackageManifest {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &Version {
        &self.version
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum PackageComponentType {
    #[serde(rename = "gate")]
    Gate,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "connection")]
    Connection,
}

impl PackageManifest {}
