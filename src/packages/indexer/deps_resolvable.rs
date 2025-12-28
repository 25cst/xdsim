use semver::{Version, VersionReq};

pub trait DepsResolvable {
    fn get_dependencies(&self, name: &str, version: &Version) -> Option<Vec<(&str, &VersionReq)>>;

    fn get_versions(&self, name: &str) -> Option<Vec<&Version>>;
}
