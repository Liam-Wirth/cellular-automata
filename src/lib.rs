#![warn(clippy::all, rust_2018_idioms)]
pub mod app;
pub mod conway;
pub use app::ConwaySim;
pub use consts::*;
pub use conway::conway_map::Map;

#[derive(Default)]
pub struct RunStatistics {
    pub births: u32,
    pub deaths: u32,
    pub generations: u32,
    pub population: u32,
}
impl RunStatistics {
    pub fn new() -> Self {
        RunStatistics {
            births: 0,
            deaths: 0,
            generations: 0,
            population: 0,
        }
    }
}
/// DECLARATION OF CONSTS

pub mod consts {
    pub const ZOOM_SENSITIVITY: f32 = 0.1;

    pub const CELL_MAX: f32 = 50.0;
    pub const CELL_MIN: f32 = 0.1;
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct Pos(pub i32, pub i32);

#[doc = "This will be where the classifications of different simulation modes/styles will exist, largely separations between two dimensional and elementary automaton, but might get more broad"]
#[derive(
    Copy, Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize, serde::Serialize, Default,
)]
pub enum RunModes {
    #[default]
    TwoDimensional,
    Elementary,
}

pub trait UserInterface {
    fn update_menu_bar(&mut self, ctx: &egui::Context);
    fn update_side_panel(&mut self, ctx: &egui::Context);
}

pub struct Viewport {
    pub rect: egui::Rect,
    pub scale: f32,         // Scale factor for zooming
    pub offset: egui::Vec2, // Offset for panning
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            rect: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::ZERO),
            scale: 1.0,
            offset: egui::Vec2::default(),
        }
    }
}
