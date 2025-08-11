use crate::consts::*;
use crate::conway;
use crate::RunModes;
use crate::UserInterface;
use crate::Viewport;
use conway::conway_map;
use eframe::egui;
use egui::Id;

#[derive(Default)]
pub struct MouseState {
    pub is_dragging: bool,
    pub last_pos: Option<egui::Pos2>,
}

use egui::LayerId;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ConwaySim {
    // TODO: Move conway_map::Map to its own file, keeping the original implementation of map underneath conway_map, but also create an interface for more generalized behavior
    map: conway::conway_map::Map,
    running: bool,
    filename: String,
    reset: bool,
    rect: Option<egui::Rect>,
    // Example stuff:
    label: String,
    fps: f32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    view_stats: bool,
    first_run: bool,
    mode: RunModes,
    #[serde(skip)] // This how you opt-out of serialization of a field
    viewport: Viewport,
    #[serde(skip)]
    show_help: bool,
    #[serde(skip)]
    show_about: bool,
}

// TODO: implement feature so that the user can click and drag on the main view window to move
// their view around instead of using sliders, cause sliders are janky as fuck

//generate documentation
impl Default for ConwaySim {
    fn default() -> Self {
        let mut map = conway_map::Map::new();
        map.gen_random();
        Self {
            // Example stuff:
            map,
            running: false,
            label: "Cellular Automata".to_owned(),
            filename: "".to_owned(),
            rect: None,
            fps: 0.0,
            value: 2.7,
            view_stats: false,
            reset: false,
            first_run: true,
            mode: RunModes::default(),
            viewport: Viewport::default(),
            show_help: false,
            show_about: false,
        }
    }
}

impl ConwaySim {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
    fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            // Space bar to toggle play/pause
            if i.key_pressed(egui::Key::Space) {
                self.running = !self.running;
            }
            
            // R key to generate random pattern
            if i.key_pressed(egui::Key::R) && !self.running {
                self.map.gen_random();
                if self.first_run {
                    self.map.cache_initial_state();
                }
            }
            
            // C key to clear
            if i.key_pressed(egui::Key::C) {
                self.map.clear();
                self.running = false;
            }
            
            // S key to save state
            if i.key_pressed(egui::Key::S) && !self.running {
                if !self.reset {
                    self.map.cache_initial_state();
                }
                self.reset = false;
            }
            
            // Z key to revert/restore
            if i.key_pressed(egui::Key::Z) {
                self.map.restore_initial_state();
                self.running = false;
            }
            
            // G key to toggle gridlines
            if i.key_pressed(egui::Key::G) {
                self.map.lines = !self.map.lines;
            }
            
            // Arrow keys for navigation
            let nav_speed = 10.0;
            if i.key_pressed(egui::Key::ArrowLeft) {
                self.map.x_axis -= nav_speed as i32;
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                self.map.x_axis += nav_speed as i32;
            }
            if i.key_pressed(egui::Key::ArrowUp) {
                self.map.y_axis -= nav_speed as i32;
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                self.map.y_axis += nav_speed as i32;
            }
            
            // Plus/Minus keys for speed control
            if i.key_pressed(egui::Key::Equals) && self.map.fps < 60 {
                self.map.fps += 1;
                self.map.update_speed();
            }
            if i.key_pressed(egui::Key::Minus) && self.map.fps > 1 {
                self.map.fps -= 1;
                self.map.update_speed();
            }
        });
    }

    fn handle_mouse_events(&mut self, ctx: &egui::Context) {
        // Handle mouse interactions for panning and cell editing
        ctx.input(|i| {
            if let Some(_pos) = i.pointer.interact_pos() {
                // TODO: Implement click-to-toggle cells
                // TODO: Implement drag-to-pan functionality
            }
        });
    }

    fn update_simulation(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let viewport_rect = ui.available_rect_before_wrap();
            self.viewport.rect = viewport_rect;

            let gridline_layer: egui::LayerId =
                LayerId::new(egui::Order::Foreground, Id::from("gridlines"));
            let painter = egui::Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );
            let line_painter = egui::Painter::new(
                ui.ctx().clone(),
                gridline_layer,
                ui.available_rect_before_wrap(),
            );
            ui.expand_to_include_rect(painter.clip_rect());
            //ui.expand_to_include_rect(line_painter.clip_rect());
            self.rect = Some(painter.clip_rect());
            //Logic that actually draws the screen I think
            let mut shapes: Vec<egui::Shape> = if self.map.light_mode {
                vec![egui::Shape::rect_filled(
                    self.rect.unwrap(),
                    egui::CornerRadius::ZERO,
                    egui::Color32::WHITE,
                )]
            } else {
                vec![egui::Shape::rect_filled(
                    self.rect.unwrap(),
                    egui::CornerRadius::ZERO,
                    egui::Color32::BLACK,
                )]
            };
            self.map.generate_cells(&mut shapes, self.rect.unwrap());
            painter.extend(shapes);
            if self.running {
                self.map.update();
            }
            if self.map.lines {
                let mut lines = vec![egui::Shape::Noop];
                self.map.draw_lines(self.rect.unwrap(), &mut lines);
                line_painter.extend(lines);
            }
        });
        if ctx.input(|i| i.pointer.secondary_clicked() && i.pointer.is_decidedly_dragging()) {
            println!("Dragging and such");
        }

        if self.view_stats {
            egui::Window::new("Stats").show(ctx, |ui| {
                ui.label("TODO :(");
            });
        }
    }
}
impl UserInterface for ConwaySim {
    fn update_side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("control_panel")
            .resizable(true)
            .default_width(280.0)
            .width_range(250.0..=400.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    // Header
                    ui.add_space(8.0);
                    ui.heading("🧬 Cellular Automata");
                    ui.add_space(12.0);
                    
                    // Simulation Controls Section
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("⏯ Simulation").strong());
                        ui.add_space(6.0);
                        
                        // Play/Pause with better styling
                        let play_button_text = if self.running { "⏸ Pause" } else { "▶ Play" };
                        let play_button = egui::Button::new(play_button_text)
                            .min_size(egui::vec2(100.0, 32.0));
                        
                        if ui.add(play_button).clicked() {
                            self.running = !self.running;
                        }
                        
                        ui.add_space(8.0);
                        
                        // Speed control with better labeling
                        ui.label("Speed (FPS)");
                        ui.add(
                            egui::Slider::new(&mut self.map.fps, 1..=60)
                                .step_by(1.0)
                                .show_value(true)
                        );
                        self.map.update_speed();
                    });
                    
                    ui.add_space(8.0);
                    
                    // Generation Controls Section
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("🎲 Generation").strong());
                        ui.add_space(6.0);
                        
                        ui.horizontal(|ui| {
                            ui.add_enabled_ui(!self.running, |ui| {
                                if ui.add(egui::Button::new("🎲 Random")).clicked() {
                                    self.map.gen_random();
                                    if self.first_run {
                                        self.map.cache_initial_state();
                                    }
                                }
                            });
                            
                            if ui.add(egui::Button::new("🗑 Clear")).clicked() {
                                self.map.clear();
                                self.running = false;
                            }
                        });
                        
                        ui.add_space(4.0);
                        
                        // Scarcity control
                        ui.label("Cell Density");
                        ui.add(
                            egui::Slider::new(&mut self.map.rand_scarcity, 0..=10)
                                .step_by(1.0)
                                .show_value(false)
                                .custom_formatter(|n, _| {
                                    match n as i32 {
                                        0..=2 => "Dense".to_string(),
                                        3..=5 => "Medium".to_string(),
                                        6..=8 => "Sparse".to_string(),
                                        _ => "Very Sparse".to_string(),
                                    }
                                })
                        );
                    });
                    
                    ui.add_space(8.0);
                    
                    // State Management Section
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("💾 State").strong());
                        ui.add_space(6.0);
                        
                        ui.horizontal(|ui| {
                            ui.add_enabled_ui(!self.running, |ui| {
                                if ui.add(egui::Button::new("💾 Save"))
                                    .on_hover_text("Save current state to restore later")
                                    .clicked() 
                                {
                                    if !self.reset {
                                        self.map.cache_initial_state();
                                    }
                                    self.reset = false;
                                }
                            });
                            
                            if ui.add(egui::Button::new("↶ Revert"))
                                .on_hover_text("Restore saved state")
                                .clicked() 
                            {
                                self.map.restore_initial_state();
                                self.running = false;
                            }
                        });
                    });
                    
                    ui.add_space(8.0);
                    
                    // View Controls Section
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("👁 View").strong());
                        ui.add_space(6.0);
                        
                        // Cell size with better formatting
                        ui.label("Cell Size");
                        ui.add(
                            egui::Slider::new(&mut self.map.cell_size, CELL_MIN..=CELL_MAX)
                                .step_by(0.1)
                                .show_value(true)
                                .custom_formatter(|n, _| format!("{:.1}px", n))
                        );
                        
                        ui.add_space(4.0);
                        
                        // Board size
                        ui.label("Board Size");
                        ui.add(
                            egui::Slider::new(&mut self.map.map_size, 10..=500)
                                .step_by(1.0)
                                .show_value(true)
                                .custom_formatter(|n, _| format!("{}×{}", n as i32, n as i32))
                        );
                        
                        ui.add_space(8.0);
                        
                        // View options
                        ui.horizontal(|ui| {
                            let grid_text = if self.map.lines { "🔲 Hide Grid" } else { "⊞ Show Grid" };
                            if ui.add(egui::Button::new(grid_text)).clicked() {
                                self.map.lines = !self.map.lines;
                            }
                            
                            if ui.add(egui::Button::new("🎯 Center"))
                                .on_hover_text("Center the view")
                                .clicked() 
                            {
                                if let Some(rect) = self.rect {
                                    self.map.center_cells(rect);
                                }
                            }
                        });
                    });
                    
                    ui.add_space(8.0);
                    
                    // Navigation Section (improved viewport controls)
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("🧭 Navigation").strong());
                        ui.add_space(6.0);
                        
                        ui.label("Horizontal Position");
                        ui.add(
                            egui::Slider::new(&mut self.map.x_axis, -1000..=1000)
                                .step_by(1.0)
                                .show_value(true)
                        );
                        
                        ui.add_space(4.0);
                        
                        ui.label("Vertical Position");
                        ui.add(
                            egui::Slider::new(&mut self.map.y_axis, -1000..=1000)
                                .step_by(1.0)
                                .show_value(true)
                        );
                        
                        ui.add_space(6.0);
                        ui.small("💡 Tip: Click and drag on the simulation to pan around");
                    });
                    
                    // Statistics section (placeholder for future)
                    ui.add_space(8.0);
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("📊 Statistics").strong());
                        ui.add_space(6.0);
                        ui.label("Coming soon...");
                        // TODO: Add generation count, population, etc.
                    });
                });
            });
    }

    fn update_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel")
            .exact_height(40.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                egui::MenuBar::new().ui(ui, |ui| {
                    // App title and status
                    ui.label(egui::RichText::new("🧬 Cellular Automata").strong().size(16.0));
                    
                    ui.separator();
                    
                    // Status indicator
                    let status_text = if self.running { "🟢 Running" } else { "⏸ Paused" };
                    let status_color = if self.running { 
                        egui::Color32::from_rgb(34, 139, 34) 
                    } else { 
                        egui::Color32::from_rgb(255, 165, 0) 
                    };
                    ui.colored_label(status_color, status_text);
                    
                    // FPS display
                    ui.separator();
                    ui.label(format!("⚡ {}fps", self.map.fps));
                    
                    // Push remaining items to the right
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Dark/Light mode toggle
                        egui::widgets::global_theme_preference_buttons(ui);
                        
                        ui.separator();
                        
                        // Help button
                        if ui.button("❓ Help").clicked() {
                            self.show_help = true;
                        }
                        
                        // About button  
                        if ui.button("ℹ About").clicked() {
                            self.show_about = true;
                        }
                        
                        // File menu (desktop only)
                        let is_web = cfg!(target_arch = "wasm32");
                        if !is_web {
                            ui.separator();
                            ui.menu_button("📁 File", |ui| {
                                if ui.button("💾 Export Pattern").clicked() {
                                    // TODO: Export current pattern
                                }
                                if ui.button("📂 Import Pattern").clicked() {
                                    // TODO: Import pattern
                                }
                                ui.separator();
                                if ui.button("❌ Quit").clicked() {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                            });
                        }
                    });
                });
            });
    }
}

impl eframe::App for ConwaySim {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input first
        self.handle_keyboard_input(ctx);
        self.handle_mouse_events(ctx);

        self.map.light_mode = ctx.style().visuals == egui::Visuals::light();
        ctx.request_repaint();
        
        self.update_side_panel(ctx);
        self.update_menu_bar(ctx);
        self.update_simulation(ctx);
        
        // Show help and about dialogs if requested
        self.show_help_dialog(ctx);
        self.show_about_dialog(ctx);
    }
}

impl ConwaySim {
    fn show_help_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_help {
            return;
        }
        
        let mut close_dialog = false;
        
        egui::Window::new("🔧 Keyboard Shortcuts")
            .open(&mut self.show_help)
            .default_width(400.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Controls");
                ui.add_space(8.0);
                
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Simulation").strong());
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("Space");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Play/Pause");
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("R");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Generate Random");
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("C");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Clear Grid");
                        });
                    });
                });
                
                ui.add_space(8.0);
                
                ui.group(|ui| {
                    ui.label(egui::RichText::new("State Management").strong());
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("S");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Save State");
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("Z");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Restore State");
                        });
                    });
                });
                
                ui.add_space(8.0);
                
                ui.group(|ui| {
                    ui.label(egui::RichText::new("View").strong());
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("G");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Toggle Gridlines");
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("Arrow Keys");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Navigate");
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("=/−");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Speed Control");
                        });
                    });
                });
                
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);
                
                if ui.button("Close").clicked() {
                    close_dialog = true;
                }
            });
            
        if close_dialog {
            self.show_help = false;
        }
    }

    fn show_about_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_about {
            return;
        }
        
        let mut close_dialog = false;
        
        egui::Window::new("ℹ About")
            .open(&mut self.show_about)
            .default_width(450.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("🧬 Cellular Automata");
                    ui.add_space(8.0);
                    ui.label("A Conway's Game of Life simulator");
                    ui.add_space(12.0);
                });
                
                ui.group(|ui| {
                    ui.label(egui::RichText::new("About Conway's Game of Life").strong());
                    ui.add_space(4.0);
                    ui.label("The Game of Life is a cellular automaton devised by mathematician John Horton Conway in 1970. It consists of a grid of cells which, based on a few mathematical rules, can live, die or multiply.");
                    ui.add_space(8.0);
                    
                    ui.label(egui::RichText::new("Rules:").strong());
                    ui.label("• Any live cell with 2-3 neighbors survives");
                    ui.label("• Any dead cell with exactly 3 neighbors becomes alive");
                    ui.label("• All other live cells die, all other dead cells stay dead");
                });
                
                ui.add_space(12.0);
                
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Built with").strong());
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("🦀 Rust");
                        ui.separator();
                        ui.label("🖼 egui");
                        ui.separator();
                        ui.label("🌐 WebAssembly");
                    });
                });
                
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);
                
                if ui.button("Close").clicked() {
                    close_dialog = true;
                }
            });
            
        if close_dialog {
            self.show_about = false;
        }
    }
}
