use eframe::egui;

use crate::{reader::{self, Config}, stat_manager, App, GraphType, ScenarioState, SearchSort, SelectedScreen};

impl App {
    pub fn central_panel(&mut self, ui: &mut egui::Ui) {
        match &self.general_data {
            Err(e) => {
                ui.label(format!("Could not load data: {}. Set stats path and restart", e));
                self.draw_stats_path_config(ui);
            },
            Ok(_general_data) => {
                ui.horizontal(|ui| {
                    if ui.button("General").clicked() {
                        self.screen = SelectedScreen::GeneralData;
                    }
                    if ui.button("Watch run").clicked() {
                        self.screen = SelectedScreen::WatchingRun(None);
                    }
                    if ui.button("Config").clicked() {
                        self.clear_buffers();
                        self.page_buffers[0].clone_from(&self.config.stats_path);
                        self.screen = SelectedScreen::Config;
                    }
                });
                ui.separator();
                self.current_screen(ui);
            },
        };
    }

    pub fn current_screen(&mut self, ui: &mut egui::Ui) {
        self.screen = match self.screen {
            SelectedScreen::GeneralData => {
                ui.label(format!("Scenario plays: {}", self.general_data.as_ref().unwrap().scenario_plays));
                ui.label(format!("Unique scenarios: {}", self.general_data.as_ref().unwrap().scenarios.len()));
                SelectedScreen::GeneralData
            },
            SelectedScreen::ScenarioData(ref scenario_state) => {
                let mut scenario_state = scenario_state.clone();
                let scenario_data = self.general_data.as_ref().unwrap().scenarios.get(&scenario_state.name).unwrap();
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Graphs:");
                        if ui.button("Score/Time").clicked() {
                            scenario_state.current_graph = GraphType::ScoreTime;
                            match stat_manager::generate_plot(&scenario_state, scenario_data) {
                                Ok(path) => scenario_state.plot_path = Some(path),
                                Err(e) => println!("Failed to generate plot: {e}"), // add popup here
                            };
                        }
                    });
                    ui.vertical(|ui| {
                        ui.heading(&scenario_state.name);
                        ui.label(format!("Plays: {}", scenario_data.plays.len()));
                    });
                });
                match &scenario_state.plot_path {
                    Some(path) => {
                        ui.image(format!("file://{}", path));
                    },
                    None => {
                        ui.label("No plot generated");
                    },
                };
                // ui.add_sized(egui::Vec2::new(550.0, 500.0), egui::Image::new(egui::include_image!("../plots/first.png")));
                SelectedScreen::ScenarioData(scenario_state)
            },
            SelectedScreen::Config => {
                ui.add_space(10.0);
                self.draw_stats_path_config(ui);
                if ui.checkbox(&mut self.config.always_show_search_results, "Always show search results").changed() {
                    reader::save_config(&self.config).unwrap();
                    self.update_search();
                }
                ui.horizontal(|ui| {
                    ui.label("Search result length");
                    if ui.add(egui::DragValue::new(&mut self.config.num_search_results).range(1..=100)).changed() {
                        reader::save_config(&self.config).unwrap();
                        self.update_search();
                    }
                });
                ui.add_space(20.0);
                if ui.button("Reset to defaults").clicked() {
                    let prev_path = self.config.stats_path.clone();
                    self.config = Config::default();
                    if reader::validate_stats_path(&prev_path).is_ok() {
                        self.config.stats_path = prev_path;
                    }
                    reader::save_config(&self.config).unwrap();
                    self.update_search();
                }
                SelectedScreen::Config
            },
            SelectedScreen::WatchingRun(run) => {
                SelectedScreen::WatchingRun(run)
            },
        };
    }

    pub fn right_panel(&mut self, ui: &mut egui::Ui) {
        match &self.general_data {
            Err(_) => (),
            Ok(_general_data) => {
                ui.label("Search scenario");
                if ui.text_edit_singleline(&mut self.search_buffer).changed() {
                    self.update_search();
                }
                ui.horizontal(|ui| {
                    ui.label("Sort:");
                    if ui.selectable_label(self.search_sort == SearchSort::None, "None").clicked() {
                        self.search_sort = SearchSort::None;
                        self.update_search();
                    }
                    if ui.selectable_label(self.search_sort == SearchSort::Plays, "Plays").clicked() {
                        self.search_sort = SearchSort::Plays;
                        self.update_search();
                    }
                });
                self.search_results.iter().for_each(|scen| {
                    if ui.button(scen).clicked() {
                        println!("{}", scen);
                        self.screen = SelectedScreen::ScenarioData(ScenarioState {
                            name: scen.clone(),
                            current_graph: GraphType::None,
                            plot_path: None,
                        });
                    }
                });
            }
        }
    }

    fn draw_stats_path_config(&mut self, ui: &mut egui::Ui) {
        ui.label("Path to stats folder");
        ui.label("This should be under FPSAimTrainer, although you can have it copied somewhere else if you like");
        ui.horizontal(|ui| {
            if ui.text_edit_singleline(&mut self.page_buffers[0]).lost_focus() {
                self.action_response = reader::validate_stats_path(&self.page_buffers[0]);
                if self.action_response.is_ok() {
                    self.config.stats_path.clone_from(&self.page_buffers[0]);
                    reader::save_config(&self.config).unwrap();
                }
            }
            match &self.action_response {
                Ok(msg) => ui.label(egui::RichText::new(msg).color(egui::Color32::LIGHT_GREEN)),
                Err(e) => ui.label(egui::RichText::new(e.to_string()).color(egui::Color32::LIGHT_RED)),
            };
        });
    }
}
