use semver::{Version, VersionReq};

use crate::{
    common::world::ComponentVersion,
    packages::{
        indexer::{
            component::PackageIndexBuilder,
            deps_resolver::{DepsResolveRequest, deps_resolver},
        },
        loader::indexed::component::IndexComponentLoader,
    },
    sim::{CreateBlankWorld, CreateDefaultGate, RegisterNewGateOutputByIndex, master::WorldState},
};

#[test]
pub fn create_not_gate() {
    let (index, res) = PackageIndexBuilder::new()
        .add_roots(&[dirs::data_dir().unwrap().join("xdsim/packages/components/")])
        .build();

    res.unwrap();

    let to_load = deps_resolver(
        &index,
        &[DepsResolveRequest::new(
            "testlib".to_string(),
            VersionReq::parse("0.1.0").unwrap(),
        )],
    )
    .unwrap();

    let loaded_libs = IndexComponentLoader::load_all(index, to_load).unwrap();

    let mut world = WorldState::new_blank(CreateBlankWorld {
        data_handles: loaded_libs.data,
        gate_handles: loaded_libs.gates,
    });

    world
        .create_default_gate(CreateDefaultGate {
            gate: ComponentVersion {
                package: "testlib".to_string(),
                version: Version::parse("0.1.0").unwrap(),
                component: "not".to_string(),
            },
        })
        .unwrap();
}

#[test]
pub fn tick_not_gate_no_panic() {
    let (index, res) = PackageIndexBuilder::new()
        .add_roots(&[dirs::data_dir().unwrap().join("xdsim/packages/components/")])
        .build();

    res.unwrap();

    let to_load = deps_resolver(
        &index,
        &[DepsResolveRequest::new(
            "testlib".to_string(),
            VersionReq::parse("0.1.0").unwrap(),
        )],
    )
    .unwrap();

    let loaded_libs = IndexComponentLoader::load_all(index, to_load).unwrap();

    let mut world = WorldState::new_blank(CreateBlankWorld {
        data_handles: loaded_libs.data,
        gate_handles: loaded_libs.gates,
    });

    world
        .create_default_gate(CreateDefaultGate {
            gate: ComponentVersion {
                package: "testlib".to_string(),
                version: Version::parse("0.1.0").unwrap(),
                component: "not".to_string(),
            },
        })
        .unwrap();

    world.tick_all().unwrap();
}
