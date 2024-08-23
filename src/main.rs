use std::error::Error;
use eframe::egui;

use reader::{GeneralData, Config};

mod reader;
mod stat_manager;
mod ui;

const NEW_STRING: String = String::new();

#[derive(Clone)]
enum SelectedScreen {
    Loading,
    Config,
    GeneralData,
    ScenarioData(ScenarioState),
    WatchingRun(Option<()>), // need some struct for watched scenario
}

#[derive(Clone)]
enum GraphType {
    None,
    ScoreTime,
}

#[derive(PartialEq)]
enum SearchSort {
    None,
    Plays,
}

#[derive(Clone)]
struct ScenarioState {
    name: String,
    current_graph: GraphType,
    plot_path: Option<String>,
}

struct App {
    general_data: Option<GeneralData>,
    config: Config,
    screen: SelectedScreen,
    action_response: Result<String, Box<dyn Error>>,
    search_buffer: String,
    page_buffers: [String; 5],
    search_results: Vec<String>,
    search_sort: SearchSort,
}

impl App {
    fn new() -> Self {
        Self {
            general_data: None,
            config: reader::get_config().unwrap(), // should add error handling
            screen: SelectedScreen::Loading,
            action_response: Ok(String::new()),
            search_buffer: String::new(),
            page_buffers: [NEW_STRING; 5],
            search_results: Vec::new(),
            search_sort: SearchSort::None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        match self.update_data() {
            Ok(()) => (),
            Err(e) => self.add_err_popup(format!("Could not update data: {}", e)),
        };
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel(ui);
        });
        egui::SidePanel::right("scen_switch").show(ctx, |ui| {
            self.right_panel(ui);
        });
    }
}

impl App {
    fn update_data(&mut self) -> Result<(), Box<dyn Error>> {
        if self.general_data.is_none() {
            self.general_data = Some(reader::get_general_data()?);
            self.screen = SelectedScreen::GeneralData;
        }
        Ok(())
    }

    fn update_search(&mut self) {
        self.search_results = match self.search_buffer.is_empty() && !self.config.always_show_search_results {
            true => Vec::new(),
            false => stat_manager::get_scen_search_results(self),
        };
    }

    fn clear_buffers(&mut self) {
        self.page_buffers.iter_mut().for_each(|buf| buf.clear());
        self.action_response = Ok(String::new());
    }

    fn add_err_popup(&mut self, msg: String) {
        println!("TODO error found: {}", msg);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    eframe::run_native(
        "Vax Analyst",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::new(App::new()))
        },
    )).unwrap();
    Ok(())
}
