use std::fmt::Display;

use semver::{Version, VersionReq};

/// Requirement for component, support rangers and wildcards
/// e.g. >=0.1.0 or 0.1.*
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ComponentVersionReq {
    pub package: String,
    pub version_req: VersionReq,
    pub component: String,
}

impl Display for ComponentVersionReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}-{}::{}",
            self.package, self.version_req, self.component
        ))
    }
}

/// A concrete component identifier
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ComponentVersion {
    pub package: String,
    pub version: Version,
    pub component: String,
}

impl Display for ComponentVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}-{}::{}",
            self.package, self.version, self.component
        ))
    }
}

/*
#[derive(Hash, PartialEq, Eq)]
pub struct ComponentLibPatchId {
    pub package: String,
    pub component: String,
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

#[derive(Hash, PartialEq, Eq)]
pub struct ComponentLibMinorId {
    pub package: String,
    pub component: String,
    pub major: u16,
    pub minor: u16,
}

#[derive(Hash, PartialEq, Eq)]
pub struct ComponentLibMajorId {
    pub package: String,
    pub component: String,
    pub major: u16,
}

#[derive(Hash, PartialEq, Eq)]
pub struct ComponentLibNameOnlyId {
    pub package: String,
    pub component: String,
}

impl ComponentLibPatchId {
    pub fn into_minor(self) -> ComponentLibMinorId {
        ComponentLibMinorId {
            package: self.package,
            component: self.component,
            major: self.major,
            minor: self.minor,
        }
    }
}
*/
