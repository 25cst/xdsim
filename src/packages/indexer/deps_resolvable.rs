use semver::{Version, VersionReq};

/// a package index that holds package in a name-version-dependencies format
pub trait DepsResolvable {
    fn get_dependencies(&self, name: &str, version: &Version) -> Option<Vec<(&str, &VersionReq)>>;
    fn get_versions(&self, name: &str) -> Option<Vec<&Version>>;
}
