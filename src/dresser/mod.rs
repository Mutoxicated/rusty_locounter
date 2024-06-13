use egui::{Color32, Vec2};
use tinyfiledialogs;

use crate::app::App;

pub struct Dresser {
    app:App,
    file_dialog_open:bool
}

impl Dresser {
    pub fn new(cdir:&str) -> Self {
        Self { app: App::new(cdir), file_dialog_open: false }
    }
}

// for egui
impl eframe::App for Dresser {
    //fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::Window::new("LOC Counter")
            .current_pos([0.0,50.0])
            .constrain(true)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .show(ctx, |ui| {
                ui.set_min_size(Vec2::new(787.0, 700.0));

                ui.label("Choose a path to your project");
                let but = ui.button("Open");
                if but.clicked() && !self.file_dialog_open {
                    self.file_dialog_open = true;

                    let selected = tinyfiledialogs::select_folder_dialog("Rusty Locounter", self.app.get_current_path());
                    if let Some(str) = selected {
                        self.file_dialog_open = false;
                        self.app.set_path(str.as_str());
                    }
                }
                
                if let Some(path) = self.app.get_path() {
                    ui.colored_label(Color32::GRAY, path);
                }

                let action = ui.button("Action");
                if action.clicked() {
                    self.app.action();
                }
                let results = self.app.results.as_ref();
                if results.is_none() {
                    return;
                }
                let results = results.unwrap();
                match results {
                    Ok(result) => {
                        ui.heading("Results");
                        ui.colored_label(Color32::GREEN, format!("Lines of Code: {}", result.loc));
                    }
                    Err(error) => {
                        ui.colored_label(Color32::RED, error.as_str());
                    }
                }
            });
    }
}