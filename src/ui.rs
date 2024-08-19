use eframe::egui;

use crate::{stat_manager, App, ScenarioState, GraphType, SelectedScreen};

impl App {
    pub fn central_panel(&mut self, ui: &mut egui::Ui) {
        match &self.general_data {
            None => {
                ui.label("Loading data");
            },
            Some(general_data) => {
                ui.horizontal(|ui| {
                    if ui.button("General").clicked() {
                        self.screen = SelectedScreen::GeneralData;
                    }
                    if ui.button("Watch run").clicked() {
                        self.screen = SelectedScreen::WatchingRun(None);
                    }
                    if ui.button("Config").clicked() {
                        self.screen = SelectedScreen::Config;
                    }
                });
                ui.separator();
                match self.screen.clone() {
                    SelectedScreen::GeneralData => {
                        ui.label(format!("Scenario plays: {}", general_data.scenario_plays));
                        ui.label(format!("Unique scenarios: {}", general_data.scenarios.len()));
                    },
                    SelectedScreen::ScenarioData(mut scenario_state) => {
                        let scenario_data = general_data.scenarios.get(&scenario_state.name).unwrap();
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label("Graphs:");
                                if ui.button("Score/Time").clicked() {
                                    scenario_state.current_graph = GraphType::ScoreTime;
                                    stat_manager::generate_plot(&scenario_state, scenario_data);
                                }
                            });
                            ui.vertical(|ui| {
                                ui.heading(&scenario_state.name);
                                ui.label(format!("Plays: {}", scenario_data.plays.len()));
                            });
                        });
                        ui.add_sized(egui::Vec2::new(550.0, 500.0), egui::Image::new(egui::include_image!("../plots/first.png")));
                    },
                    SelectedScreen::Loading => {
                        // might delete this
                    },
                    SelectedScreen::Config => {
                        ui.label("Will put config here");
                    },
                    SelectedScreen::WatchingRun(_run) => {

                    },
                };
            },
        };
    }

    pub fn right_panel(&mut self, ui: &mut egui::Ui) {
        match &self.general_data {
            None => (),
            Some(general_data) => {
                ui.label("Search scenario");
                if ui.text_edit_singleline(&mut self.search_buffer).changed() {
                    self.search_results = match self.search_buffer.is_empty() {
                        true => Vec::new(),
                        false => stat_manager::get_scen_search_results(&general_data, &self.search_buffer),
                    };
                }
                self.search_results.iter().for_each(|scen| {
                    if ui.button(scen).clicked() {
                        println!("{}", scen);
                        self.screen = SelectedScreen::ScenarioData(ScenarioState {
                            name: scen.clone(),
                            current_graph: GraphType::None,
                        });
                    }
                });
            }
        }
    }
}
