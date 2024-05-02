#![warn(clippy::all, rust_2018_idioms)]
pub mod app;
pub mod conway;
pub use app::ConwaySim;
pub use conway::conway_map::Map;

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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Pos(pub i32, pub i32);

impl Default for Pos {
    fn default() -> Self {
        Pos(0, 0)
    }
}
#[doc = "This will be where the classifications of different simulation modes/styles will exist, largely separations between two dimensional and elementary automaton, but might get more broad"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize, serde::Serialize)]
pub enum RunModes {
    TwoDimensional,
    Elementary,
}

impl Default for RunModes {
    fn default() -> Self {
        RunModes::TwoDimensional //For now, I want the default mode to just be the most familiar CA, Conway's life and friends
    }
}

pub trait UserInterface {
    fn update_menu_bar(&self, ctx: &egui::Context);
    fn update_side_panel(&mut self, ctx: &egui::Context);
}
