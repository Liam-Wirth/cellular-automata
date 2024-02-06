use crate::connway;
use connway::connway_map;
use connway::connway_map::Map;
use eframe::egui;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    map: connway::connway_map::Map,
    running: bool,
    filename: String,
    rect: Option<egui::Rect>,
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}
// TODO: implement feature so that the user can click and drag on the main view window to move
// their view around instead of using sliders, cause sliders are janky as fuck
impl Default for App {
    fn default() -> Self {
        let mut map = connway_map::Map::new();
        map.gen_random();
        Self {
            // Example stuff:
            map,
            running: false,
            label: "Hello World!".to_owned(),
            filename: "".to_owned(),
            rect: None,
            value: 2.7,
        }
    }
}

impl App {
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
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        self.map.light_mode = (ctx.style().visuals == egui::Visuals::light());
        ctx.request_repaint();
        egui::SidePanel::left("Menu").show(ctx, |ui| {
            ui.add(
                egui::Slider::new(&mut self.map.cell_size, 0.1..=50.0)
                    .step_by(0.1)
                    .orientation(egui::SliderOrientation::Horizontal)
                    .text("Cell Size"),
            );
            ui.add(
                egui::Slider::new(&mut self.map.rand_scarcity, 0..=10)
                    .step_by(1.0)
                    .orientation(egui::SliderOrientation::Horizontal)
                    .text("Scarcity of generated cells, higher = more sparsely placed"),
            );

            ui.add(
                egui::Slider::new(&mut self.map.fps, 1..=60)
                    .step_by(1.0)
                    .orientation(egui::SliderOrientation::Horizontal)
                    .text("FPS"),
            );
            self.map.update_speed();

            ui.add(
                egui::Slider::new(&mut self.map.x_axis, -1000..=1000)
                    .step_by(1.0)
                    .orientation(egui::SliderOrientation::Horizontal)
                    .text("X Axis"),
            );
            ui.add(
                egui::Slider::new(&mut self.map.y_axis, -1000..=1000)
                    .step_by(1.0)
                    .orientation(egui::SliderOrientation::Horizontal)
                    .text("Y Axis"),
            );
            ui.add(
                egui::Slider::new(&mut self.map.map_size, 10..=500)
                    .step_by(1.0)
                    .orientation(egui::SliderOrientation::Horizontal)
                    .text("Board Size"),
            );

            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Toggle")).clicked() {
                    self.running = !self.running;
                }
                if ui.add(egui::Button::new("Random")).clicked() {
                    self.map.gen_random();
                    self.map.center_cells(self.rect.unwrap());
                }
                if ui.add(egui::Button::new("Clean")).clicked() {
                    self.map.clean();
                }
                if ui.add(egui::Button::new("Gridlines")).clicked() {
                    self.map.lines = !self.map.lines; 
                }
            });
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = egui::Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );
            ui.expand_to_include_rect(painter.clip_rect());
            self.rect = Some(painter.clip_rect());
            let mut shapes = vec![egui::Shape::rect_filled(
                self.rect.unwrap(),
                egui::Rounding::ZERO,
                egui::Color32::WHITE,
            )];
            if self.map.light_mode {
                shapes = vec![egui::Shape::rect_filled(
                    self.rect.unwrap(),
                    egui::Rounding::ZERO,
                    egui::Color32::WHITE,
                )];
            } else {
                shapes = vec![egui::Shape::rect_filled(
                    self.rect.unwrap(),
                    egui::Rounding::ZERO,
                    egui::Color32::BLACK,
                )];
            }
            self.map.generate_cells(&mut shapes, self.rect.unwrap());
            painter.extend(shapes);
            if self.running {
                self.map.update();
            }
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
