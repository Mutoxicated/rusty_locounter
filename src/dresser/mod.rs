use egui::{lerp, Color32, ScrollArea, Vec2};

use crate::app::App;

pub struct Dresser {
    app:App,
}

impl Dresser {
    pub fn new(cdir:&str) -> Self {
        Self { app: App::new(cdir) }
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

                // SETTINGS
                ui.label("Choose a path to your project");
                let but = ui.button("Open");
                if but.clicked() {

                    let selected = tinyfiledialogs::select_folder_dialog("Rusty Locounter", self.app.get_current_path());
                    if let Some(str) = selected {
                        self.app.set_path(str.as_str());
                    }
                }
                
                if let Some(path) = &self.app.get_path() {
                    let path = path.to_owned().to_owned();
                    self.app.set_current_path(path.as_str());
                    ui.colored_label(Color32::GRAY, path);
                }

                ui.menu_button("Add custom extensions", |menu| {

                    let but = menu.button("+");
                    if but.clicked() {
                        self.app.add_extension("");
                    }

                    let iter = self.app.iterate_extensions();
                    if iter.is_none() {
                        return;
                    }
                    let iter = iter.unwrap();
                    
                    for i in 0..iter.len() {
                        let mut exit = false;
                        menu.horizontal(|h| {
                            h.set_width(160.0);
                            let label = h.label("Name");
                            let _ = h.text_edit_singleline(self.app.get_extension(i))
                                .labelled_by(label.id);
                            let remove = h.button("-");
                            if remove.clicked() {
                                self.app.remove_extension(i);
                                exit = true;
                            }
                        });
                        if exit {
                            break;
                        }
                    }
                });

                ui.menu_button("Prohibit folders", |menu| {

                    let but = menu.button("+");
                    if but.clicked() {
                        self.app.add_folder("");
                    }
                    
                    menu.vertical(|v| {
                        let iter = self.app.iterate_folders();

                        if iter.is_none() {
                            return;
                        }
                        let iter = iter.unwrap();
                        for i in 0..iter.len() {
                            let mut exit = false;
                            v.horizontal(|h| {
                                h.set_width(160.0);
                                let label = h.label("Name");
                                let _ = h.text_edit_singleline(self.app.get_folder(i))
                                    .labelled_by(label.id);
                                let remove = h.button("-");
                                if remove.clicked() {
                                    self.app.remove_folder(i);
                                    exit = true;
                                }
                            });
                            if exit {
                                break;
                            }
                        }
                    });
                });

                // ACTION
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
                        ui.label("Files:");
                        ScrollArea::vertical()
                            .max_height(180.0)
                            .max_width(200.0)
                            .auto_shrink(true)
                            .show_rows(ui, 180.0, result.files.len(),|sa, _| {
                                sa.with_layout(egui::Layout::top_down(egui::Align::LEFT), |l| {
                                    for file in &result.files {
                                        l.horizontal(|h| {
                                            let file_info = String::from(&file.name)+":"+file.loc.to_string().as_str();
                                            h.label(file_info);
                                        });
                                    }
                                });
                        });
                    }
                    Err(error) => {
                        ui.colored_label(Color32::RED, error.as_str());
                    }
                }
            });
    }
}