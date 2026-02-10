use std::default;

use iced::Length::Shrink;
use iced::widget::{button, row, text};
use iced::Element;

use semver::{Version, VersionReq};

use crate::{
    common::world::{ComponentId, ComponentVersion, GateInputSocket, GateOutputSocket},
    packages::{
        indexer::{
            component::PackageIndexBuilder,
            deps_resolver::{DepsResolveRequest, deps_resolver},
        },
        loader::indexed::component::IndexComponentLoader,
    },
    world::sim::{WorldState, requests::*},
};

fn create_world() -> (WorldState, ComponentId) {
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

    let gate = world
        .create_default_gate(CreateDefaultGate {
            gate: ComponentVersion {
                package: "testlib".to_string(),
                version: Version::parse("0.1.0").unwrap(),
                component: "not".to_string(),
            },
        })
        .unwrap();
    (world, gate)
}

fn create_gate(world: &mut WorldState) -> ComponentId {
    world
        .create_default_gate(CreateDefaultGate {
            gate: ComponentVersion {
                package: "testlib".to_string(),
                version: Version::parse("0.1.0").unwrap(),
                component: "not".to_string(),
            },
        })
        .unwrap()
}

struct State {
    gates: Vec<ComponentId>,
    world: WorldState
}
impl State {
    pub fn getstring(&self) -> String {
        self.gates.iter().map(|x| self.world.get_gate(x).unwrap().get_type().to_string()).reduce(|a, b| { a + "," + &b }).unwrap_or("[]".into())
    }
}
impl Default for State {
    fn default() -> Self {
        let (w, g) = create_world();
        Self { gates: vec![g], world: w }
    }
}

fn view(counter: &Option<State>) -> Element<'_, Message> {
    row![button(text(counter.as_ref().map(|c| { c.getstring() }).unwrap_or_default())).on_press(Message::Increment)].into()
}

fn update(counter: &mut Option<State>, message: Message) {
    match message {
        Message::Increment => {
            let s = counter.get_or_insert_default();
            s.gates.push(create_gate(&mut s.world));
        }
    }
}

pub fn main() -> iced::Result {
    let (w, g) = create_world();
    
    iced::run(update, view)
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

