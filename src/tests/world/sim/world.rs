use semver::{Version, VersionReq};

use crate::{
    common::world::{ComponentVersion, GateConsumerSocket, GateProducerSocket},
    packages::{
        indexer::{
            component::PackageIndexBuilder,
            deps_resolver::{DepsResolveRequest, deps_resolver},
        },
        loader::indexed::component::IndexComponentLoader,
    },
    world::sim::{WorldState, requests::*},
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
pub fn tick_not_gate_no_producer() {
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

#[test]
pub fn tick_not_gate_multiple() {
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

    let not_gate = world
        .create_default_gate(CreateDefaultGate {
            gate: ComponentVersion {
                package: "testlib".to_string(),
                version: Version::parse("0.1.0").unwrap(),
                component: "not".to_string(),
            },
        })
        .unwrap();

    world
        .connect_gates(ConnectIOSockets {
            producer_socket: GateProducerSocket::new(not_gate, 0),
            consumer_socket: GateConsumerSocket::new(not_gate, 0),
        })
        .unwrap();

    macro_rules! get_data {
        () => {
            unsafe {
                *(world
                    .get_buffer(&GateProducerSocket::new(not_gate, 0))
                    .unwrap()
                    .get_data_ptr() as *const u8)
            }
        };
    }

    assert_eq!(dbg!(get_data!()), 0);
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!()), 0);
    world.tick_all().unwrap();
    assert_eq!(dbg!(get_data!()), 0);
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!()), 0);
    world.tick_all().unwrap();
    assert_eq!(dbg!(get_data!()), 0);
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!()), 0);
    world.tick_all().unwrap();
    assert_eq!(dbg!(get_data!()), 0);
}

#[test]
pub fn tick_not_gate_disconnect_connect() {
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

    let not1 = world
        .create_default_gate(CreateDefaultGate {
            gate: ComponentVersion {
                package: "testlib".to_string(),
                version: Version::parse("0.1.0").unwrap(),
                component: "not".to_string(),
            },
        })
        .unwrap();
    let not2 = world
        .create_default_gate(CreateDefaultGate {
            gate: ComponentVersion {
                package: "testlib".to_string(),
                version: Version::parse("0.1.0").unwrap(),
                component: "not".to_string(),
            },
        })
        .unwrap();

    world
        .connect_gates(ConnectIOSockets {
            producer_socket: GateProducerSocket::new(not1, 0),
            consumer_socket: GateConsumerSocket::new(not1, 0),
        })
        .unwrap();

    macro_rules! get_data {
        ($x : expr) => {
            unsafe {
                *(world
                    .get_buffer(&GateProducerSocket::new($x, 0))
                    .unwrap()
                    .get_data_ptr() as *const u8)
            }
        };
    }

    assert_eq!(dbg!(get_data!(not1)), 0);
    assert_eq!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_eq!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);

    world
        .connect_gates(ConnectIOSockets {
            producer_socket: GateProducerSocket::new(not1, 0),
            consumer_socket: GateConsumerSocket::new(not2, 0),
        })
        .unwrap();

    println!("=== connect 1");
    world.tick_all().unwrap();
    assert_eq!(dbg!(get_data!(not1)), 0);
    assert_eq!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_eq!(dbg!(get_data!(not1)), 0);
    assert_eq!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);
    println!("---");

    world
        .disconnect_gates(DisconnectIOSockets {
            producer_socket: GateProducerSocket::new(not1, 0),
            consumer_socket: GateConsumerSocket::new(not2, 0),
        })
        .unwrap();

    println!("=== disconnect 1");
    world.tick_all().unwrap();
    assert_eq!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_eq!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);

    world
        .disconnect_gates(DisconnectIOSockets {
            producer_socket: GateProducerSocket::new(not1, 0),
            consumer_socket: GateConsumerSocket::new(not1, 0),
        })
        .unwrap();

    println!("=== disconnect 2");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_ne!(dbg!(get_data!(not2)), 0);

    println!("=== connect 2");

    world
        .connect_gates(ConnectIOSockets {
            producer_socket: GateProducerSocket::new(not1, 0),
            consumer_socket: GateConsumerSocket::new(not2, 0),
        })
        .unwrap();

    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_eq!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_eq!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_eq!(dbg!(get_data!(not2)), 0);
    println!("---");
    world.tick_all().unwrap();
    assert_ne!(dbg!(get_data!(not1)), 0);
    assert_eq!(dbg!(get_data!(not2)), 0);
}
