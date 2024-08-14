use std::error::Error;
use eframe::egui;

mod reader;
mod stat_manager;

struct App {
    general_data: reader::GeneralData,
    selected_scenario: Option<String>,
    search_buffer: String,
    search_results: Vec<String>,
}

impl App {
    fn new(general_data: reader::GeneralData) -> Self {
        Self {
            general_data,
            selected_scenario: None,
            search_buffer: String::new(),
            search_results: Vec::new(),
        }
    }
}

impl App {
    fn central_panel(&mut self, ui: &mut egui::Ui) {
        match self.selected_scenario.clone() {
            Some(name) => {
                if ui.button("Back").clicked() {
                    self.selected_scenario = None;
                }
                ui.label(&name);

                let scenario = self.general_data.scenarios.get(&name).unwrap();

                ui.label(format!("Plays: {}", scenario.plays.len()));
            },
            None => {
                ui.label(format!("Scenario plays: {}", self.general_data.scenario_plays));
                ui.label(format!("Unique scenarios: {}", self.general_data.scenarios.len()));
            },
        };
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel(ui)
        });
        egui::SidePanel::right("scen_switch").show(ctx, |ui| {
            ui.label("Search scen");
            if ui.text_edit_singleline(&mut self.search_buffer).changed() {
                self.search_results = match self.search_buffer.is_empty() {
                    true => Vec::new(),
                    false => stat_manager::get_scen_search_results(&self.general_data, &self.search_buffer),
                };
            }
            self.search_results.iter().for_each(|scen| {
                if ui.button(scen).clicked() {
                    println!("{}", scen);
                    self.selected_scenario = Some(scen.clone());
                }
            });
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let general_data = reader::get_general_data()?;

    eframe::run_native(
        "Vax Analyst",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::new(App::new(general_data)))
        },
    )).unwrap();

    Ok(())
}
