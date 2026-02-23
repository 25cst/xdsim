use std::default;
use std::ops::{Add, Div, Mul, Sub};

use iced::Color;
use iced::Length::Fill;
use iced::widget::canvas::LineDash;
use iced::{Element, Theme, mouse};
use iced::widget::{button, canvas, row, text};

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

fn create_world() -> WorldState {
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

    WorldState::new_blank(CreateBlankWorld {
        data_handles: loaded_libs.data,
        gate_handles: loaded_libs.gates,
    })
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
    world: WorldState,
}
impl State {
    pub fn getstring(&self) -> String {
        self.gates
            .iter()
            .map(|x| self.world.get_gate(x).unwrap().get_type().to_string())
            .reduce(|a, b| a + "," + &b)
            .unwrap_or("[]".into())
    }
}
impl Default for State {
    fn default() -> Self {
        Self {
            gates: vec![],
            world: create_world(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2f {
    pub x: f32, pub y: f32
}
impl From<Vec2f> for iced::Point {
    fn from(value: Vec2f) -> Self {
        Self { x: value.x, y: value.y }
    }
}
impl From<Vec2f> for iced::Size {
    fn from(value: Vec2f) -> Self {
        Self { width: value.x, height: value.y }
    }
}
impl From<iced::Point> for Vec2f {
    fn from(value: iced::Point) -> Self {
        Self { x: value.x, y: value.y }
    }
}
impl From<iced::Size> for Vec2f {
    fn from(value: iced::Size) -> Self {
        Self { x: value.width, y: value.height }
    }
}
impl<T> Add<T> for Vec2f where Vec2f: From<T> {
    type Output = Vec2f;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: Vec2f = rhs.into();
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl<T> Sub<T> for Vec2f where Vec2f: From<T> {
    type Output = Vec2f;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Vec2f = rhs.into();
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl Mul<f32> for Vec2f {
    type Output = Vec2f;
    fn mul(self, rhs: f32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Div<f32> for Vec2f {
    type Output = Vec2f;
    fn div(self, rhs: f32) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect2f {
    pub topleft: Vec2f,
    pub size: Vec2f
}
impl From<iced::Rectangle> for Rect2f {
    fn from(value: iced::Rectangle) -> Self {
        Self { topleft: value.position().into(), size: value.size().into() }
    }
}
impl From<Rect2f> for iced::Rectangle {
    fn from(value: Rect2f) -> Self {
        Self::new(value.topleft.into(), value.size.into())
    }
}

#[derive(Debug, Clone, Copy)]
struct FillStyle {
    color: Color
}
impl From<FillStyle> for canvas::fill::Fill {
    fn from(value: FillStyle) -> Self {
        Self { style: canvas::Style::Solid(value.color), rule: canvas::fill::Rule::EvenOdd }
    }
}
impl Default for FillStyle {
    fn default() -> Self {
        Self { color: Color::TRANSPARENT }
    }
}

#[derive(Debug, Clone, Copy)]
struct StrokeStyle {
    color: Color,
    width: f32
}
impl <'a> From<StrokeStyle> for canvas::stroke::Stroke<'a> {
    fn from(value: StrokeStyle) -> Self {
        Self { 
            style: canvas::Style::Solid(value.color), 
            width: value.width, 
            line_cap: canvas::LineCap::Round, 
            line_join: canvas::LineJoin::Round, 
            line_dash: canvas::LineDash::default() 
        }
    }
}
impl Default for StrokeStyle {
    fn default() -> Self {
        Self { color: Color::TRANSPARENT, width: 1.0 }
    }
}

trait DrawOnto {
    fn draw_onto(&self, f: &mut iced::widget::canvas::Frame, bounds: Rect2f);
}

struct Circle {
    pub center: Vec2f, 
    pub radius: f32, 
    pub fill: Option<FillStyle>,
    pub stroke: Option<StrokeStyle>
}
impl Circle {
    pub fn new(center: Vec2f, radius: f32) -> Self {
        Self {center, radius, fill: None, stroke: None}
    }
    pub fn with_fill(self, fill: Option<FillStyle>) -> Self {
        Self { center: self.center, radius: self.radius, fill, stroke: self.stroke }
    }
    pub fn with_stroke(self, stroke: Option<StrokeStyle>) -> Self {
        Self { center: self.center, radius: self.radius, fill: self.fill, stroke }
    }
}
impl DrawOnto for Circle {
    fn draw_onto(&self, f: &mut iced::widget::canvas::Frame, _bounds: Rect2f) {
        let Circle { center, radius, fill, stroke } = self;
        let circle = canvas::path::Path::circle((*center).into(), *radius);
        if let Some(fill) = fill { f.fill(&circle, *fill); }
        if let Some(stroke) = stroke { f.stroke(&circle, *stroke); }
    }
}

struct Rectangle {
    pub topleft: Vec2f,
    pub size: Vec2f,
    pub fill: Option<FillStyle>,
    pub stroke: Option<StrokeStyle>
}
impl Rectangle {
    pub fn new(topleft: Vec2f, size: Vec2f) -> Self {
        Self {topleft, size, fill: None, stroke: None}
    }
    pub fn with_fill(self, fill: Option<FillStyle>) -> Self {
        Self { topleft: self.topleft, size: self.size, fill, stroke: self.stroke }
    }
    pub fn with_stroke(self, stroke: Option<StrokeStyle>) -> Self {
        Self { topleft: self.topleft, size: self.size, fill: self.fill, stroke }
    }
}
impl DrawOnto for Rectangle {
    fn draw_onto(&self, f: &mut iced::widget::canvas::Frame, _bounds: Rect2f) {
        let Rectangle { topleft, size, fill, stroke } = self;
        let rect = canvas::path::Path::rectangle((*topleft).into(), (*size).into());
        if let Some(fill) = fill { f.fill(&rect, *fill); }
        if let Some(stroke) = stroke { f.stroke(&rect, *stroke); }
    }
}

enum GraphicsElem {
    // TODO: tbh this could be done with traits and OOP but nah for now
    //  can always refactor later
    Circle(Circle),
    Rectangle(Rectangle),
}
impl From<Circle> for GraphicsElem {
    fn from(value: Circle) -> Self {
        Self::Circle(value)
    }
}
impl From<Rectangle> for GraphicsElem {
    fn from(value: Rectangle) -> Self {
        Self::Rectangle(value)
    }
}
impl DrawOnto for GraphicsElem {
    fn draw_onto(&self, f: &mut iced::widget::canvas::Frame, bounds: Rect2f) {
        match self {
            GraphicsElem::Circle(c) => c.draw_onto(f, bounds),
            GraphicsElem::Rectangle(r) => r.draw_onto(f, bounds),
        }
    }
}

struct GateRenderer<'a> {
    state: &'a State
}
impl<'a, MsgT> canvas::Program<MsgT> for GateRenderer<'a> {
    type State = ();
    fn draw(
            &self,
            _state: &Self::State,
            renderer: &iced::Renderer,
            _theme: &iced::Theme,
            bounds: iced::Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry<iced::Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let bounds: Rect2f = bounds.into();
        for (i, _c) in self.state.gates.iter().enumerate() {
            let center = bounds.topleft + (bounds.size * (i as f32 + 0.5) / (self.state.gates.len() as f32));
            if i % 2 == 0 {
                Circle::new(center, 4.5)
                    .with_fill(Some(FillStyle { color: Color::WHITE }))
                    .draw_onto(&mut frame, bounds);
            } else {
                let size = Vec2f {x: 9.0, y: 9.0};
                Rectangle::new(center - size / 2.0, size).with_fill(Some(FillStyle { color: Color::BLACK })).draw_onto(&mut frame, bounds);
            }
        }
        vec![frame.into_geometry()]
    }
}

fn view(counter: &State) -> Element<'_, Message> {
    button(canvas(GateRenderer{state: counter}).width(400).height(400)).padding(0).on_press(Message::Increment).into()
    
    // row![
    //     button(text(
    //         counter.getstring()
    //     ))
    //     .on_press(Message::Increment)
    // ]
    // .into()
}

fn update(counter: &mut State, message: Message) {
    match message {
        Message::Increment => {
            let s = counter;
            s.gates.push(create_gate(&mut s.world));
        }
    }
}

pub fn main() -> iced::Result {
    iced::run(update, view)
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}
