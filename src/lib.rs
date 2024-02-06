#![warn(clippy::all, rust_2018_idioms)]
pub mod connway;
pub mod app;
pub use app::App;
pub use connway::connway_map::Map;
