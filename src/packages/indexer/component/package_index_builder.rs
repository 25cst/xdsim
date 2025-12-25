use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use semver::Version;

use crate::packages::indexer::{
    self,
    component::{
        PackageManifest,
        package_index::{Package, PackageIndex},
    },
};

/// a package index builder is an incomplete package index
/// it contains errors information which will be removed on build
/// and can be modified
pub struct PackageIndexBuilder {
    packages: HashMap<String, Vec<Package>>,
    /// errors are returned on build()
    errors: Vec<indexer::Error>,
}

impl PackageIndexBuilder {
    /// create new instance
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
            errors: Vec::new(),
        }
    }

    /// build self to a PackageIndex
    /// returns PackageIndex, and the errors
    /// since the indexer can build with errors present
    /// PackageIndex and Result are both returned
    pub fn build(mut self) -> (PackageIndex, Result<(), indexer::Error>) {
        let mut packages_cleaned = HashMap::new();

        for (name, mut package) in self.packages.into_iter() {
            match package.len() {
                0 => unreachable!(
                    "this should not happen, a package is added to index only when a definition is found, please report this error"
                ),
                1 => {
                    let _ = packages_cleaned.insert(name, package.remove(0));
                }
                _ => self.errors.push(indexer::Error::MultipleDefinitions {
                    name: package[0].get_name().to_string(),
                    paths: package
                        .into_iter()
                        .map(|package_def| package_def.into_root())
                        .collect(),
                }),
            }
        }

        (
            PackageIndex::from_packages(packages_cleaned),
            if self.errors.is_empty() {
                Ok(())
            } else {
                Err(indexer::Error::NewIndex {
                    errors: self.errors,
                })
            },
        )
    }

    /// read packages using the specified folders as package root
    pub fn add_roots(mut self, paths: &[PathBuf]) -> Self {
        macro_rules! wrap_fs_op {
            ($ex: expr, $p: expr) => {
                match $ex {
                    Ok(res) => res,
                    Err(e) => {
                        self.errors.push(indexer::Error::Fs {
                            path: PathBuf::from($p),
                            reason: e.to_string(),
                        });
                        continue;
                    }
                }
            };
        }

        // read root path where immediate childrens are directories
        // where the name of each dir is a package
        for root_path in paths {
            if !wrap_fs_op!(fs::exists(root_path), root_path) {
                self.errors.push(indexer::Error::IndexMissingDir {
                    index_path: root_path.clone(),
                });
                continue;
            }

            // read the dir of a package
            // the immediate childrens are versions of the package
            // the name of the dir is the full version name of the package version
            for package in wrap_fs_op!(fs::read_dir(root_path), root_path) {
                let package = wrap_fs_op!(package, root_path);
                let package_path = package.path();

                let mut package_builder = Package::new(package_path.clone());

                if !wrap_fs_op!(fs::metadata(&package_path), package_path).is_dir() {
                    continue;
                }

                // read a single version in a package
                // package.toml is inside the dir
                for version in wrap_fs_op!(fs::read_dir(&package_path), package_path) {
                    let version = wrap_fs_op!(version, &package_path);
                    let version_path = version.path();
                    let manifest_path = version_path.join("package.toml");

                    if !wrap_fs_op!(fs::metadata(&manifest_path), &manifest_path).is_file() {
                        continue;
                    }

                    let manifest_content = wrap_fs_op!(fs::read(&manifest_path), &manifest_path);
                    let manifest: PackageManifest = match toml::from_slice(&manifest_content) {
                        Ok(res) => res,
                        Err(e) => {
                            self.errors.push(indexer::Error::ManifestParse {
                                manifest_path: manifest_path.clone(),
                                reason: e.to_string(),
                            });
                            continue;
                        }
                    };

                    if let Err(e) = package_builder.add_version(&version_path, manifest) {
                        self.errors.push(e);
                    }
                }

                if let Err(e) = self.add_package(package_builder, &package_path) {
                    self.errors.push(e);
                }
            }
        }

        self
    }
}

impl PackageIndexBuilder {
    fn add_package(&mut self, package: Package, package_root: &Path) -> Result<(), indexer::Error> {
        let expected_name = package_root
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        if package.get_name() != expected_name {
            return Err(indexer::Error::NameMismatch {
                expected: expected_name.to_string(),
                got: package.get_name().to_string(),
                package_root: package_root.to_path_buf(),
            });
        }

        if package.is_empty() {
            let (name, package_root) = package.destruct();
            return Err(indexer::Error::NoVersions { name, package_root });
        }

        self.packages
            .entry(package.get_name().to_string())
            .or_default()
            .push(package);

        Ok(())
    }
}

impl Package {
    pub fn add_version(
        &mut self,
        version_path: &Path,
        manifest: PackageManifest,
    ) -> Result<(), indexer::Error> {
        let path_version_string = version_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let expected_version = match Version::parse(&path_version_string) {
            Ok(v) => v,
            Err(e) => {
                return Err(indexer::Error::BadVersionString {
                    version_root: version_path.to_path_buf(),
                    got: path_version_string,
                    reason: e.to_string(),
                });
            }
        };

        if expected_version != *manifest.get_version() {
            return Err(indexer::Error::VersionMismatch {
                expected: expected_version.to_string(),
                got: manifest.get_version().to_string(),
                version_root: version_path.to_path_buf(),
            });
        }

        self.insert(expected_version, manifest);
        Ok(())
    }
}
