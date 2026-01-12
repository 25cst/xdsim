use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use semver::Version;

use crate::packages::indexer::{component::PackageManifest, deps_resolvable::DepsResolvable};

/// an index of all packages
#[cfg_attr(feature = "devel", derive(Debug))]
pub struct PackageIndex {
    packages: HashMap<String, Package>,
}

impl PackageIndex {
    /// internal: construct the struct
    pub fn from_packages(packages: HashMap<String, Package>) -> Self {
        Self { packages }
    }

    pub fn get_package(&self, name: &str) -> Option<&Package> {
        self.packages.get(name)
    }

    /*
    pub fn get_data(&self) -> Vec<(ComponentLibPatchId, PathBuf)> {

    }
    */
}

#[cfg_attr(feature = "devel", derive(Debug))]
pub struct Package {
    name: String,
    package_root: PathBuf,
    versions: HashMap<Version, PackageManifest>,
}

impl Package {
    pub fn new(package_root: PathBuf) -> Self {
        Self {
            name: package_root
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            package_root,
            versions: HashMap::new(),
        }
    }

    /// add a version to a package
    pub fn insert(&mut self, version: Version, manifest: PackageManifest) {
        self.versions.insert(version, manifest);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_root(&self) -> &Path {
        &self.package_root
    }

    /// destruct into name, root
    pub fn destruct(self) -> (String, PathBuf) {
        (self.name, self.package_root)
    }

    pub fn into_name(self) -> String {
        self.name
    }

    pub fn into_root(self) -> PathBuf {
        self.package_root
    }

    /// returns true if the package has no versions (how?)
    pub fn is_empty(&self) -> bool {
        self.versions.is_empty()
    }

    pub fn get_version(&self, version: &Version) -> Option<&PackageManifest> {
        self.versions.get(version)
    }

    pub fn list_versions(&self) -> Vec<&Version> {
        self.versions.keys().collect()
    }
}

impl DepsResolvable for PackageIndex {
    fn get_dependencies(
        &self,
        name: &str,
        version: &Version,
    ) -> Option<Vec<(&str, &semver::VersionReq)>> {
        Some(
            self.packages
                .get(name)?
                .get_version(version)?
                .get_dependencies()
                .iter()
                .map(|(name, version)| (name.as_str(), version))
                .collect(),
        )
    }

    fn get_versions(&self, name: &str) -> Option<Vec<&Version>> {
        Some(self.packages.get(name)?.list_versions())
    }
}
