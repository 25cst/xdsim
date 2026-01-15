use std::{
    collections::{BTreeMap, HashMap},
    env::consts::DLL_EXTENSION,
    path::PathBuf,
    rc::Rc,
};

use semver::Version;

use crate::{
    common::world::ComponentVersion,
    packages::{
        destructor::{self, DestructRequest, DestructedData, DestructedGate},
        indexer::{self, component::PackageComponentType},
        loader::{self, LibraryHandle, manager::LoadManager},
    },
};

type PackageName = String;
type PackageVersion = Version;
type LibName = String;

type DestructedGateHandles =
    HashMap<PackageName, BTreeMap<PackageVersion, HashMap<LibName, Rc<DestructedGate>>>>;
type DestructedDataHandles =
    HashMap<PackageName, BTreeMap<PackageVersion, HashMap<LibName, Rc<DestructedData>>>>;

struct LoadedEntry {
    pub variant: PackageComponentType,
    pub handle: LibraryHandle,
    pub path: PathBuf,
}

/// library loading utility for situations where:
/// - you are trying to load component packages
/// - you already have an index of the packages
///
/// fields are public, for convenience of destructing:
/// the struct can only be created, it does not have any self referencing functions
pub struct IndexComponentLoader {
    pub gates: DestructedGateHandles,
    pub data: DestructedDataHandles,
}

impl IndexComponentLoader {
    /// given an index and a list of packages to load,
    /// load all libraries that the packages contains into memory
    ///
    /// this does not destruct the libraries
    pub fn load_all(
        index: indexer::component::PackageIndex,
        packages_to_load: HashMap<String, Vec<Version>>,
    ) -> Result<Self, loader::Error> {
        let mut errors = Vec::new();

        let mut loaded_index = HashMap::new();

        for (package_name, versions_to_load) in packages_to_load {
            let mut package_map = HashMap::new();

            let package = match index.get_package(&package_name) {
                Some(pkg) => pkg,
                None => {
                    errors.push(loader::Error::MissingPackage { name: package_name });
                    continue;
                }
            };

            for version in versions_to_load {
                let mut version_map = HashMap::new();

                let manifest = match package.get_version(&version) {
                    Some(manifest) => manifest,
                    None => {
                        errors.push(loader::Error::MissingPackageVersion {
                            name: package_name.clone(),
                            version: version.clone(),
                        });
                        continue;
                    }
                };

                let libs_to_load = manifest.get_provides();
                let version_root = package.get_root().join(version.to_string());

                for (name, variant) in libs_to_load {
                    let lib_path = version_root.join(name).with_extension(DLL_EXTENSION);

                    let lib = match LoadManager::load_with_path(lib_path.clone()) {
                        Ok(loaded) => loaded,
                        Err(e) => {
                            errors.push(e);
                            continue;
                        }
                    };

                    version_map.insert(
                        name.clone(),
                        LoadedEntry {
                            variant: *variant,
                            handle: lib,
                            path: lib_path,
                        },
                    );
                }

                package_map.insert(version, version_map);
            }

            loaded_index.insert(package_name.clone(), package_map);
        }

        /// maintain the structure of the hashmap (includes empty entries)
        /// extract a map for a component from the loaded index
        fn destruct_component<T>(
            destruct: fn(DestructRequest) -> Result<T, destructor::Error>,
            variant: PackageComponentType,
            index: &HashMap<PackageName, HashMap<PackageVersion, HashMap<LibName, LoadedEntry>>>,
            errors: &mut Vec<loader::Error>,
        ) -> HashMap<PackageName, BTreeMap<PackageVersion, HashMap<LibName, Rc<T>>>> {
            index
                .iter()
                .map(|(package_name, versions)| {
                    (
                        package_name.clone(),
                        versions
                            .iter()
                            .map(|(version_name, version_content)| {
                                (
                                    version_name.clone(),
                                    version_content
                                        .iter()
                                        .filter_map(|(lib_name, lib_content)| {
                                            if lib_content.variant == variant {
                                                let destruct_request = DestructRequest::new(
                                                    lib_content.handle.clone(),
                                                    ComponentVersion {
                                                        package: package_name.clone(),
                                                        version: version_name.clone(),
                                                        component: lib_name.clone(),
                                                    },
                                                );

                                                match destruct(destruct_request) {
                                                    Ok(destructed) => Some((
                                                        lib_name.clone(),
                                                        Rc::new(destructed),
                                                    )),
                                                    Err(e) => {
                                                        errors.push(
                                                            loader::Error::DestructorError {
                                                                content: e.to_string(),
                                                            },
                                                        );

                                                        None
                                                    }
                                                }
                                            } else {
                                                None
                                            }
                                        })
                                        .collect::<HashMap<_, _>>(),
                                )
                            })
                            .collect::<BTreeMap<_, _>>(),
                    )
                })
                .collect::<HashMap<_, _>>()
        }

        let gates = destruct_component(
            DestructedGate::new,
            PackageComponentType::Gate,
            &loaded_index,
            &mut errors,
        );
        let data = destruct_component(
            DestructedData::new,
            PackageComponentType::Data,
            &loaded_index,
            &mut errors,
        );
        // TODO: destruct connections

        if errors.is_empty() {
            Ok(Self { gates, data })
        } else {
            Err(loader::Error::LoadAllComponentPackages { errors })
        }
    }
}
