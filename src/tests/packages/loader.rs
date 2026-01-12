use semver::{Version, VersionReq};

use crate::packages::{
    indexer::{
        component::PackageIndexBuilder,
        deps_resolver::{DepsResolveRequest, deps_resolver},
    },
    loader::indexed::component::IndexComponentLoader,
};

#[test]
fn load_single_lib() {
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

    dbg!(
        &loaded_libs
            .gates
            .get("testlib")
            .unwrap()
            .get(&Version::parse("0.1.0").unwrap())
            .unwrap()
            .keys()
    );
    dbg!(
        &loaded_libs
            .data
            .get("testlib")
            .unwrap()
            .get(&Version::parse("0.1.0").unwrap())
            .unwrap()
            .keys()
    );
}
