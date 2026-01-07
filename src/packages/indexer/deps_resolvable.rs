use semver::{Version, VersionReq};

/// a package index that holds package in a name-version-dependencies format
pub trait DepsResolvable {
    /// returns the dependencies of a package, returned in [(package name, required version)]
    fn get_dependencies(&self, name: &str, version: &Version) -> Option<Vec<(&str, &VersionReq)>>;
    /// returns all versions of the specified package
    fn get_versions(&self, name: &str) -> Option<Vec<&Version>>;
}
