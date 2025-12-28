use std::collections::HashMap;

use semver::{Version, VersionReq};

use crate::packages::indexer::{self, deps_resolvable::DepsResolvable};

#[derive(Clone)]
pub struct DepsResolveRequest {
    name: String,
    version: VersionReq,
}

impl DepsResolveRequest {
    fn new(name: String, version: VersionReq) -> Self {
        Self { name, version }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_version(&self) -> &VersionReq {
        &self.version
    }
}

/// given a list of packages that are required,
/// return a full list of packages and their dependencies that will need to be loaded
///
/// may return an indexer::Error::MissingDependencies if there are any missing dependencies
pub fn deps_resolver<I: DepsResolvable>(
    resolvable: &I,
    requests: &[&DepsResolveRequest],
) -> Result<HashMap<String, Vec<Version>>, indexer::Error> {
    let mut resolved: HashMap<String, Vec<Version>> = HashMap::new();

    // represents a dependency chain
    // the first item in the dependency chain is the actual item that is missing
    // todo: get the newest version of all packages possible
    type MissingDependency = Vec<(String, VersionReq)>;

    fn internal_resolver<I: DepsResolvable>(
        resolvable: &I,
        request_name: &str,
        request_version: &VersionReq,
        resolved: &mut HashMap<String, Vec<Version>>,
    ) -> Result<(), Vec<MissingDependency>> {
        if let Some(package_versions) = resolved.get(request_name)
            && package_versions
                .iter()
                .any(|version| request_version.matches(version))
        {
            return Ok(());
        }

        // see available versions
        let versions = match resolvable.get_versions(request_name) {
            Some(versions) => versions,
            None => {
                return Err(vec![vec![(
                    request_name.to_string(),
                    request_version.clone(),
                )]]);
            }
        };

        // pick the version to load
        let accepted_version = match versions
            .iter()
            .find(|version| request_version.matches(version))
        {
            Some(accepted_version) => accepted_version,
            None => {
                return Err(vec![vec![(
                    request_name.to_string(),
                    request_version.clone(),
                )]]);
            }
        };

        // add version to resolved list of packages
        resolved
            .entry(request_name.to_string())
            .or_default()
            .push((*accepted_version).clone());

        let mut missings = Vec::new();

        let deps = match resolvable.get_dependencies(request_name, accepted_version) {
            Some(deps) => deps,
            None => unreachable!(
                "the package with this name and version should guarantee to exist (because of previous step)"
            ),
        };

        // look for dependencies of this package
        for (dep_name, dep_version) in deps {
            if let Err(mut dep_missings) =
                internal_resolver(resolvable, dep_name, dep_version, resolved)
            {
                missings.append(&mut dep_missings);
            }
        }

        if missings.is_empty() {
            Ok(())
        } else {
            // adds the current package to the end of the dependency chain
            let request = (request_name.to_string(), request_version.clone());
            missings
                .iter_mut()
                .for_each(|chain| chain.push(request.clone()));
            Err(missings)
        }
    }

    let mut missings = Vec::new();

    for request in requests {
        if let Err(mut dep_missings) = internal_resolver(
            resolvable,
            request.get_name(),
            request.get_version(),
            &mut resolved,
        ) {
            missings.append(&mut dep_missings);
        }
    }

    if missings.is_empty() {
        Ok(resolved)
    } else {
        Err(indexer::Error::MissingDependencies {
            dependency_chains: missings,
        })
    }
}
