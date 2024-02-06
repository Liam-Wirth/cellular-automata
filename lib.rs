#![warn(clippy::all, rust_2018_idioms)]
pub mod app;
pub mod connway;
pub use app::App;
pub use connway::connway_map::Map;

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
