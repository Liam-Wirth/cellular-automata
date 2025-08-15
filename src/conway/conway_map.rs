// TODO: In the future you need to refactor this for other simulations, take out the map logic, and
// the drawing logic and isolate that in some other class in a more generalized manner, you would
// keep the Map struct, but rename it to conway_map, and then for any other simulation you would
// have it be (sim name)_Map struct or something. Then, you would use those structs to make the
// draw calls you figured out/thought about in the main, kinda "super" Map struct.

// TODO: Implement a "stamp" or blueprint feature in which the user can stamp their own pre-saved
// game of life patterns into the map? Provide some basic ones like gliders and such

// TODO: clean up this code, remove magic values
// TODO: Get better understanding of what every funciton does + add documentation for each function
// TODO: Refactor the code to be more modular, and to be more easily testable

use std::{collections::HashSet, fs};

use crate::Pos;
use egui::{vec2, Color32, Rect, CornerRadius, Shape};
use instant::{Duration, Instant};
use rand::{thread_rng, Rng};

use super::{DEFAULT_CELL_SIZE, DEFAULT_MAP_SIZE, NEIGHBORS};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConwayCell {
    Alive = 1,
    Dead = 0,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
/// Eventually will be generalized to be a "Map" struct, but for now it's just a Conway's Game of
pub struct Map {
    /// X axis of the map
    pub x_axis: i32,
    /// Y axis of the map
    pub y_axis: i32,
    /// Size of each cell in the map, will be clamped between constants CELL_MIN and CELL_MAX
    pub cell_size: f32,
    /// Size of the map, eventually I want this to be separate from our viewport
    pub map_size: i32,
    /// Speed of the simulation, in what unit? only God knows
    pub speed: u128,
    /// Frames per second
    pub fps: u32,
    ///Determines the scarcity of cells in the initial state
    pub rand_scarcity: u32,
    /// Self explanatory
    pub light_mode: bool,
    /// Whether or not to draw gridlines
    pub lines: bool,
    /// Whether to display the map as toroidal/infinite (wrapping)
    pub toroidal_display: bool,
    pub is_initial: bool,

    #[serde(skip)]
    last_frame_time: Instant,
    #[serde(skip)]
    cells: HashSet<Pos>,
    initial_state: HashSet<Pos>,
}

impl Default for Map {
    fn default() -> Self {
        Map::new()
    }
}

impl Map {
    pub fn new() -> Self {
        Self {
            fps: 10,
            speed: Map::fps_to_speed(10.0), //why the hell am I storing the "speed" value if I'm just deriving it from fps?
            cells: HashSet::new(),
            initial_state: HashSet::new(),
            last_frame_time: Instant::now(),
            map_size: DEFAULT_MAP_SIZE,
            cell_size: DEFAULT_CELL_SIZE,
            x_axis: 0,
            y_axis: 0,
            rand_scarcity: 3,
            light_mode: true,
            lines: false,
            toroidal_display: false,
            is_initial: true,
        }
    }
    pub fn update_speed(&mut self) {
        self.speed = Map::fps_to_speed(self.fps as f32);
    }
    // pub fn neighbors(&self, p: &Pos) -> usize {
    //     let mut neighbors = 0;
    //     for i in NEIGHBORS {
    //         if self.cells.contains(&Pos(p.0 + i.0, p.1 + i.1)) {
    //             neighbors += 1;
    //         }
    //     }
    //     neighbors
    // }
    // NOTE: Below, I have a new toroidal function for neighbor checking, and above, I have a more
    // naive check, maybe later look into seeing if it is possible to toggle between say "hard"
    // walls and toroidal walls that just tile
    pub fn neighbors(&self, p: &Pos) -> usize {
        NEIGHBORS.iter().fold(0, |neighbors, &i| {
            let mut neighbor_pos = Pos(p.0 + i.0, p.1 + i.1);

            neighbor_pos.0 = self.apply_periodic_boundary(neighbor_pos.0, self.map_size);
            neighbor_pos.1 = self.apply_periodic_boundary(neighbor_pos.1, self.map_size);

            neighbors + self.cells.contains(&neighbor_pos) as usize
        })
    }

    ///Generates the random initial state for the map,
    /// Bases the way the initial state is off of the "rand_scarcity" value
    pub fn gen_random(&mut self) {
        self.clear();
        for y in 0..=self.map_size - 4 {
            for x in 0..=self.map_size - 4 {
                let mut rng = thread_rng();
                let probability = rng.gen_range(0..=self.rand_scarcity);
                if probability == 1 {
                    self.cells.insert(Pos(x, y));
                }
            }
        }
        //self.cache_initial_state();
    }
    pub fn cache_initial_state(&mut self) {
        self.initial_state.clone_from(&self.cells);
        //basically anytime this has been called, AND update has not been called, we can garuntee we
        //are in the "initial" state of the app
        self.is_initial = true;
    }
    pub fn restore_initial_state(&mut self) {
        self.cells.clone_from(&self.initial_state);
        self.is_initial = true;
    }

    pub fn clear(&mut self) {
        self.cells = HashSet::new();
    }

    /// Toggle a cell at the given position (alive -> dead, dead -> alive)
    pub fn toggle_cell(&mut self, pos: Pos) {
        if self.cells.contains(&pos) {
            self.cells.remove(&pos);
        } else {
            self.cells.insert(pos);
        }
    }

    /// Check if a cell is alive at the given position
    pub fn is_cell_alive(&self, pos: Pos) -> bool {
        self.cells.contains(&pos)
    }

    /// Set a cell to be alive
    pub fn set_cell_alive(&mut self, pos: Pos) {
        self.cells.insert(pos);
    }

    /// Set a cell to be dead
    pub fn set_cell_dead(&mut self, pos: Pos) {
        self.cells.remove(&pos);
    }

    /// Convert screen coordinates to grid position
    pub fn screen_to_grid(&self, screen_pos: egui::Pos2, rect: Rect) -> Option<Pos> {
        // Calculate center offset
        let center_offset_x = rect.width() / 2.0 - (self.map_size as f32 * self.cell_size) / 2.0;
        let center_offset_y = rect.height() / 2.0 - (self.map_size as f32 * self.cell_size) / 2.0;
        
        // Calculate the grid position accounting for centering
        let relative_x = screen_pos.x - rect.min.x - center_offset_x;
        let relative_y = screen_pos.y - rect.min.y - center_offset_y;
        
        let grid_x = (relative_x / self.cell_size) as i32 + self.x_axis;
        let grid_y = (relative_y / self.cell_size) as i32 + self.y_axis;
        
        // In toroidal mode, wrap coordinates to map bounds
        if self.toroidal_display {
            let wrapped_x = ((grid_x % self.map_size) + self.map_size) % self.map_size;
            let wrapped_y = ((grid_y % self.map_size) + self.map_size) % self.map_size;
            Some(Pos(wrapped_x, wrapped_y))
        } else {
            Some(Pos(grid_x, grid_y))
        }
    }

    /// Convert grid position to screen coordinates
    pub fn grid_to_screen(&self, grid_pos: Pos, rect: Rect) -> egui::Rect {
        // Calculate center offset
        let center_offset_x = rect.width() / 2.0 - (self.map_size as f32 * self.cell_size) / 2.0;
        let center_offset_y = rect.height() / 2.0 - (self.map_size as f32 * self.cell_size) / 2.0;
        
        let screen_x = rect.min.x + center_offset_x + (grid_pos.0 - self.x_axis) as f32 * self.cell_size;
        let screen_y = rect.min.y + center_offset_y + (grid_pos.1 - self.y_axis) as f32 * self.cell_size;
        
        egui::Rect::from_min_size(
            egui::Pos2::new(screen_x, screen_y),
            egui::Vec2::splat(self.cell_size)
        )
    }

    /// Draw a highlight over a specific cell
    pub fn draw_cell_highlight(&self, grid_pos: Pos, rect: Rect, shapes: &mut Vec<Shape>) {
        let highlight_color = if self.light_mode {
            Color32::from_rgba_unmultiplied(0, 100, 255, 100) // Blue with transparency
        } else {
            Color32::from_rgba_unmultiplied(100, 150, 255, 100) // Light blue with transparency
        };
        
        if self.toroidal_display {
            // In toroidal mode, draw highlights for all visible instances of this cell
            self.draw_cell_highlight_toroidal(grid_pos, rect, shapes, highlight_color);
        } else {
            // Standard mode: single highlight
            let cell_rect = self.grid_to_screen(grid_pos, rect);
            if rect.intersects(cell_rect) {
                self.draw_single_highlight(cell_rect, shapes, highlight_color);
            }
        }
    }

    fn draw_cell_highlight_toroidal(&self, grid_pos: Pos, rect: Rect, shapes: &mut Vec<Shape>, highlight_color: Color32) {
        // Calculate center offset
        let center_offset_x = rect.width() / 2.0 - (self.map_size as f32 * self.cell_size) / 2.0;
        let center_offset_y = rect.height() / 2.0 - (self.map_size as f32 * self.cell_size) / 2.0;
        
        // Calculate how many times we need to tile the map to fill the viewport
        let map_pixel_size = self.map_size as f32 * self.cell_size;
        
        // Calculate the range of tiles needed to cover the entire viewport
        let start_tile_x = ((rect.min.x - center_offset_x) / map_pixel_size).floor() as i32 - 1;
        let end_tile_x = ((rect.max.x - center_offset_x) / map_pixel_size).ceil() as i32 + 1;
        let start_tile_y = ((rect.min.y - center_offset_y) / map_pixel_size).floor() as i32 - 1;
        let end_tile_y = ((rect.max.y - center_offset_y) / map_pixel_size).ceil() as i32 + 1;
        
        // Draw highlights for all visible instances of this cell
        for tile_x in start_tile_x..=end_tile_x {
            for tile_y in start_tile_y..=end_tile_y {
                let tile_offset_x = tile_x as f32 * map_pixel_size;
                let tile_offset_y = tile_y as f32 * map_pixel_size;
                
                let screen_x = rect.min.x + center_offset_x + tile_offset_x + (grid_pos.0 - self.x_axis) as f32 * self.cell_size;
                let screen_y = rect.min.y + center_offset_y + tile_offset_y + (grid_pos.1 - self.y_axis) as f32 * self.cell_size;
                
                let cell_rect = Rect {
                    min: egui::Pos2::new(screen_x, screen_y),
                    max: egui::Pos2::new(screen_x + self.cell_size, screen_y + self.cell_size),
                };
                
                if rect.intersects(cell_rect) {
                    self.draw_single_highlight(cell_rect, shapes, highlight_color);
                }
            }
        }
    }

    fn draw_single_highlight(&self, cell_rect: Rect, shapes: &mut Vec<Shape>, highlight_color: Color32) {
        shapes.push(Shape::rect_filled(
            cell_rect,
            CornerRadius::ZERO,
            highlight_color,
        ));
        
        // Add a border
        shapes.push(Shape::rect_stroke(
            cell_rect,
            CornerRadius::ZERO,
            egui::Stroke::new(1.0, Color32::from_rgb(0, 100, 255)),
            egui::epaint::StrokeKind::Outside,
        ));
    }
    pub fn fps_to_speed(fps: f32) -> u128 {
        //magic number?
        Duration::new(0, (1000000000.0 / fps) as u32).as_millis()
    }
    // NOTE: This could probably be useful for the refactor
    /// How the simulation runs, this is the main function that updates the state of the map, is called once every draw thread
    /// not the fastest way to run a simulation, but could work if it's thrown into some worker thread maybe but idrc
    pub fn update(&mut self) {
        let duration_since_last_frame = Instant::now().duration_since(self.last_frame_time);
        //below line basically forces fps to work. like, it's saying "if last frame happened, but
        //is lower then our set speed, don't do SHIT!"
        if duration_since_last_frame.as_millis().lt(&self.speed) {
            return;
        }
        let mut n_cells = HashSet::new();
        let mut checked = HashSet::new();
        for cell in &self.cells {
            for step in NEIGHBORS {
                let mut xy = Pos(cell.0 + step.0, cell.1 + step.1);

                // Does toroidal checks here, basically coordinates on the edge will "wrap around"
                xy.0 = self.apply_periodic_boundary(xy.0, self.map_size);
                //Same here
                xy.1 = self.apply_periodic_boundary(xy.1, self.map_size);

                if !checked.contains(&xy) {
                    checked.insert(xy);
                    let n = self.neighbors(&xy);
                    if n == 2 && self.cells.contains(&xy) || n == 3 {
                        n_cells.insert(xy);
                    }
                }
            }
        }
        self.last_frame_time = Instant::now();
        self.cells = n_cells;
        self.is_initial = false;
    }
    // NOTE: If I end up generalizing/standardizing the way a map is implemented in some refactor
    // down the line, I should move alot of these functions to a parent mod.rs file. For now I
    // don't want to build unneccessary abstractions if it turns out I don't need them, and have
    // weird code layout with everything in tiny files when it doesn't need to be organized that
    // way.
    fn find_min(&self) -> (i32, i32) {
        let mut min_x = -1;
        let mut min_y = -1;
        for el in &self.cells {
            if min_x == -1 || el.0 < min_x {
                min_x = el.0;
            }
            if min_y == -1 || el.1 < min_y {
                min_y = el.1;
            }
        }
        (min_x, min_y)
    }
    fn find_max(&self) -> (i32, i32) {
        let mut max_x = -1;
        let mut max_y = -1;
        for el in &self.cells {
            if el.0 > max_x {
                max_x = el.0;
            }
            if el.1 > max_y {
                max_y = el.1;
            }
        }
        (max_x, max_y)
    }
    //What does this do????
    pub fn center_cells(&mut self, rect: Rect) {
        let (min_x, min_y) = self.find_min();
        let (max_x, max_y) = self.find_max();
        let mut elems_c = HashSet::new();
        if rect.max.x > rect.max.y {
            self.cell_size = ((rect.max.x - rect.min.x) as i32 / self.map_size) as f32;
        } else {
            self.cell_size = ((rect.max.y - rect.min.y) as i32 / self.map_size) as f32;
        }
        for cell in &self.cells {
            elems_c.insert(Pos(
                self.map_size / 2 - (max_x - min_x) / 2 + cell.0,
                self.map_size / 2 - (max_y - min_y) / 2 + cell.1,
            ));
        }

        self.cells = elems_c;
    }
    /// Draw grid lines that properly align with the viewport and cells
    pub fn draw_lines(&mut self, rect: Rect, shapes: &mut Vec<Shape>) {
        // Calculate stroke thickness based on cell size
        let stroke_thickness = self.exponential_easing(crate::CELL_MIN, crate::CELL_MAX, 0.1, 1.5);
        
        // Grid color based on theme
        let grid_color = if self.light_mode {
            Color32::from_gray(200)
        } else {
            Color32::from_gray(60)
        };

        // Calculate the offset for grid alignment
        let offset_x = (self.x_axis as f32 * self.cell_size) % self.cell_size;
        let offset_y = (self.y_axis as f32 * self.cell_size) % self.cell_size;

        // Draw vertical grid lines
        let num_vertical_lines = (rect.width() / self.cell_size).ceil() as i32 + 2;
        for i in 0..num_vertical_lines {
            let x = rect.min.x + (i as f32 * self.cell_size) - offset_x;
            if x >= rect.min.x - self.cell_size && x <= rect.max.x + self.cell_size {
                shapes.push(Shape::line_segment(
                    [
                        egui::Pos2::new(x, rect.min.y),
                        egui::Pos2::new(x, rect.max.y),
                    ],
                    egui::Stroke::new(stroke_thickness, grid_color),
                ));
            }
        }

        // Draw horizontal grid lines
        let num_horizontal_lines = (rect.height() / self.cell_size).ceil() as i32 + 2;
        for i in 0..num_horizontal_lines {
            let y = rect.min.y + (i as f32 * self.cell_size) - offset_y;
            if y >= rect.min.y - self.cell_size && y <= rect.max.y + self.cell_size {
                shapes.push(Shape::line_segment(
                    [
                        egui::Pos2::new(rect.min.x, y),
                        egui::Pos2::new(rect.max.x, y),
                    ],
                    egui::Stroke::new(stroke_thickness, grid_color),
                ));
            }
        }
    }
    pub fn generate_cells(&self, shapes: &mut Vec<Shape>, rect: Rect) {
        // Calculate center offset to center the map in the viewport
        let center_offset_x = rect.width() / 2.0 - (self.map_size as f32 * self.cell_size) / 2.0;
        let center_offset_y = rect.height() / 2.0 - (self.map_size as f32 * self.cell_size) / 2.0;
        
        if self.toroidal_display {
            // Toroidal display: show cells wrapping around infinitely
            self.draw_cells_toroidal(shapes, rect, center_offset_x, center_offset_y);
        } else {
            // Standard display: show cells with centering
            self.draw_cells_standard(shapes, rect, center_offset_x, center_offset_y);
        }
    }

    fn draw_cells_standard(&self, shapes: &mut Vec<Shape>, rect: Rect, center_offset_x: f32, center_offset_y: f32) {
        for c in &self.cells {
            let screen_x = rect.min.x + center_offset_x + (c.0 - self.x_axis) as f32 * self.cell_size;
            let screen_y = rect.min.y + center_offset_y + (c.1 - self.y_axis) as f32 * self.cell_size;
            
            let cell_rect = Rect {
                min: egui::Pos2::new(screen_x, screen_y),
                max: egui::Pos2::new(screen_x + self.cell_size, screen_y + self.cell_size),
            };
            
            // Only draw cells that are visible in the viewport
            if rect.intersects(cell_rect) {
                shapes.push(Shape::rect_filled(
                    cell_rect,
                    CornerRadius::ZERO,
                    if self.light_mode {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    },
                ));
            }
        }
    }

    fn draw_cells_toroidal(&self, shapes: &mut Vec<Shape>, rect: Rect, center_offset_x: f32, center_offset_y: f32) {
        // Calculate how many times we need to tile the map to fill the viewport
        let map_pixel_size = self.map_size as f32 * self.cell_size;
        
        // Calculate the range of tiles needed to cover the entire viewport
        let start_tile_x = ((rect.min.x - center_offset_x) / map_pixel_size).floor() as i32 - 1;
        let end_tile_x = ((rect.max.x - center_offset_x) / map_pixel_size).ceil() as i32 + 1;
        let start_tile_y = ((rect.min.y - center_offset_y) / map_pixel_size).floor() as i32 - 1;
        let end_tile_y = ((rect.max.y - center_offset_y) / map_pixel_size).ceil() as i32 + 1;
        
        // Draw the map tiled across the viewport with no gaps
        for tile_x in start_tile_x..=end_tile_x {
            for tile_y in start_tile_y..=end_tile_y {
                let tile_offset_x = tile_x as f32 * map_pixel_size;
                let tile_offset_y = tile_y as f32 * map_pixel_size;
                
                for c in &self.cells {
                    let screen_x = rect.min.x + center_offset_x + tile_offset_x + (c.0 - self.x_axis) as f32 * self.cell_size;
                    let screen_y = rect.min.y + center_offset_y + tile_offset_y + (c.1 - self.y_axis) as f32 * self.cell_size;
                    
                    let cell_rect = Rect {
                        min: egui::Pos2::new(screen_x, screen_y),
                        max: egui::Pos2::new(screen_x + self.cell_size, screen_y + self.cell_size),
                    };
                    
                    // Only draw cells that are visible in the viewport
                    if rect.intersects(cell_rect) {
                        shapes.push(Shape::rect_filled(
                            cell_rect,
                            CornerRadius::ZERO,
                            if self.light_mode {
                                Color32::BLACK
                            } else {
                                Color32::WHITE
                            },
                        ));
                    }
                }
            }
        }
    }
    ///Function largely exists solely for the purpose of easing the thickness of the gridlines
    ///based on the cell size
    #[allow(dead_code)]
    fn sigmoid_easing(&mut self, x_0: f32, k: f32) -> f32 {
        let exponent = -k * (self.cell_size - x_0);
        1.0 / (1.0 + exponent.exp())
    }
    /// Another easing function but this time we use exponential stuff cause I can
    fn exponential_easing(
        &mut self,
        min_cell_size: f32,
        max_cell_size: f32,
        min_thickness: f32,
        max_thickness: f32,
    ) -> f32 {
        if self.cell_size <= min_cell_size {
            return min_thickness; // Gridlines disappear when zoomed out completely
        }
        if self.cell_size > max_cell_size {
            return max_thickness; // Gridlines are thickest when zoomed in completely
        }

        let t = (self.cell_size - min_cell_size) / (max_cell_size - min_cell_size); // Normalized value between 0 and 1

        min_thickness + t * (max_thickness - min_thickness)
    }

    // TODO: Use this code, and a provided text box to allow users to make "blueprints"
    pub fn generate_from_file(&mut self, f: &str) {
        if fs::read_to_string(f).is_err() {
            println!("Error reading from file");
            return;
        };
        let contents = fs::read_to_string(f).expect("Error reading from file");

        let mut x = HashSet::new();
        for (ind, l) in contents.split('\n').enumerate() {
            for (i, c) in l.chars().enumerate() {
                if c == '#' {
                    x.insert(Pos(i as i32, ind as i32));
                }
            }
        }
        self.cells = x;
    }
    fn apply_periodic_boundary(&self, coord: i32, axis_size: i32) -> i32 {
        match coord {
            x if x < 0 => axis_size - 1,
            x if x >= axis_size => 0,
            x => x,
        }
    }
}
