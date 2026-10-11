use eframe::egui;
//use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::fs;

/// Structure of JSON data. All of the fields of Squad are of a type which
/// implements the Default trait, so that means we can derive Default for the
/// entire Squad struct.
#[derive(Serialize, Deserialize, Default)]
struct Squad {
    firepower: u8,
    range: u8,
    morale: u8,
}

/// Main Application Struct
struct JSONEditorApp {
    squad: Option<Squad>,
    file_path: Option<String>,
}

impl Default for JSONEditorApp {
    fn default() -> Self {
        Self { squad: None, file_path: None }
    }
}

impl eframe::App for JSONEditorApp {
    // ctx can be used to customize the app, changing things like font, etc.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SquadLeader Scenario Editor");

            // User clicks button "Load Scenario" to select a json file to load.
            if ui.button("Load Scenario").clicked() {
                // We load the JSON file using the rfd crate.
                // Remember: the rfd crate is bundled with eframe.
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("text", &["json"])
                    .pick_file()
                {
                    self.file_path = Some(path.display().to_string());
                    let file_content =
                        fs::read_to_string(&self.file_path.clone().unwrap())
                            .expect("Failed to read from file");
                    self.squad = serde_json::from_str(&file_content).ok();
                }
            }

            // Display and edit the squads firepower.
            if let Some(squad) = &mut self.squad {
                ui.horizontal(|ui| {
                    ui.label("Firepower: ");
                    ui.add(
                        egui::DragValue::new(&mut squad.firepower)
                            .clamp_range(1..=10) // u8 range
                            .speed(1),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Range: ");

                    ui.add(
                        egui::DragValue::new(&mut squad.range)
                            .clamp_range(1..=10) // u8 range
                            .speed(1),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Morale: ");
                    ui.add(
                        egui::DragValue::new(&mut squad.morale)
                            .clamp_range(1..=10) // u8 range
                            .speed(1),
                    );
                });

                // Save JSON file.
                if ui.button("Save JSON").clicked() {
                    if let Some(path) = &self.file_path {
                        let json = serde_json::to_string_pretty(squad)
                            .expect("Failed to serialize.");
                        fs::write(path, json).expect("Failed to save file.");
                    }
                }
            } else {
                ui.label("No JSON file loaded");
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "JSON Editor",
        options,
        Box::new(|_cc| Box::new(JSONEditorApp::default())),
    )
}
