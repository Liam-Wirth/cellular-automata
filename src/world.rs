use std::{collections::hashset, fs};

use egui::{vec2, color32, rect, rounding, shape};
use instant::{duration, instant};
use rand::{thread_rng, rng};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Pos(pub i32, pub i32);

impl Default for Pos {
    fn default() -> Self {
        Pos(0,0)
    }
}
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

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
 pub struct Map {
     
 }
