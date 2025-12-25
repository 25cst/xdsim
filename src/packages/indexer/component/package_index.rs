use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use semver::Version;

use crate::packages::indexer::component::PackageManifest;

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

    pub fn insert(&mut self, version: Version, manifest: PackageManifest) {
        self.versions.insert(version, manifest);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_root(&self) -> &Path {
        &self.package_root
    }

    /// name, root
    pub fn destruct(self) -> (String, PathBuf) {
        (self.name, self.package_root)
    }

    pub fn into_name(self) -> String {
        self.name
    }

    pub fn into_root(self) -> PathBuf {
        self.package_root
    }

    pub fn is_empty(&self) -> bool {
        self.versions.is_empty()
    }
}
