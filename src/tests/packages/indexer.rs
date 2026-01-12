use semver::VersionReq;

use crate::packages::indexer::{
    component::PackageIndexBuilder,
    deps_resolver::{DepsResolveRequest, deps_resolver},
};

#[test]
fn index_home() {
    let (index, res) = PackageIndexBuilder::new()
        .add_roots(&[dirs::data_dir().unwrap().join("xdsim/packages/components/")])
        .build();

    res.unwrap();
    dbg!(index);
}

#[test]
fn resolver_single() {
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
    dbg!(to_load);
}
