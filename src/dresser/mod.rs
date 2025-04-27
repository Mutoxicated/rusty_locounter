use egui::{Color32, ScrollArea, Ui};

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
        egui::SidePanel::left("Settings")
            .resizable(false)
            .exact_width(200.0)
            .show_separator_line(true)
            .show(ctx, |ui| {

                // SETTINGS
                ui.label("Choose a path to your project");
                let but = ui.button("Open");
                if but.clicked() {

                    let selected = tinyfiledialogs::select_folder_dialog("Rusty Locounter", self.app.get_current_path());
                    if let Some(str) = selected {
                        self.app.set_path(str.as_str());
                        self.app.set_current_path(str.as_str());
                    }
                }
                
                if let Some(path) = self.app.get_path() {
                    let path = path.to_owned();
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

                let action = ui.button("Action");
                if action.clicked() {
                    self.app.action();
                }
            });
        
        egui::SidePanel::right("Results")
            .resizable(false)
            .exact_width(620.0)
            .show(ctx, |ui| {
                if let Some(err) = &self.app.error {
                    ui.colored_label(Color32::RED, err.as_str());
                    return;
                }

                let results = self.app.results.as_ref();
                if results.is_none() {
                    return;
                }
                let results = results.unwrap();
                ui.heading("Results");
                ui.colored_label(Color32::GREEN, format!("Lines of Code: {}", results.loc));
                ui.label("Files:");
                ScrollArea::vertical()
                    .max_height(500.0)
                    .max_width(250.0)
                    .auto_shrink(false)
                    .show(ui,|sa| {
                        sa.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |l| {
                            dress_folder(results.root.as_ref().unwrap(), 0, l);
                        });
                });
                ui.heading("Extra");
                let longest = results.longest_line.as_ref().unwrap();
                ui.label("Longest line:");
                ui.label(longest.path.as_ref().unwrap());
                ui.label(longest.content.clone());
                ui.label("Size:".to_owned()+longest.size.to_string().as_str());
                
            });
    }
}

use crate::app::folder::EFolder;

fn ident(num:usize) -> String {
    let mut str = String::new();
    for _ in 0..num {
        str.push_str("   ");
    }
    str
}

fn section(num:usize) -> String {
    let mut str = String::new();
    for _ in 0..num {
        str.push_str("--");
    }
    str.push(' ');
    str
}

fn dress_folder(efolder:&EFolder, deep:usize, ui: &mut Ui) {
    let yellow = Color32::from_rgb(255,255/(deep+1) as u8,0);

    let files = efolder.get_files();
    let section = section(deep);
    let extra_tabs = &ident(deep+1);
    ui.colored_label(yellow, section+efolder.name()+": "+efolder.total_loc().to_string().as_str());
    if let Some(files) = files {
        for file in files {
            ui.horizontal(|h| {
                let file_info = String::from(&file.name)+": "+file.loc.to_string().as_str();
                let text = extra_tabs.clone();
                h.label(text+file_info.as_str());
            });
        }
    }

    let nexts = efolder.next_immut();

    for next in nexts {
        dress_folder(next, deep+1, ui);
    }
}