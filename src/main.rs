use std::error::Error;
use eframe::egui;

mod reader;
mod stat_manager;
mod ui;

#[derive(Clone)]
enum SelectedScreen {
    Loading,
    Config,
    GeneralData,
    ScenarioData(String),
    WatchingRun(Option<()>), // need some struct for watched scenario
}

struct App {
    general_data: Option<reader::GeneralData>,
    screen: SelectedScreen,
    search_buffer: String,
    search_results: Vec<String>,
}

impl App {
    fn new() -> Self {
        Self {
            general_data: None,
            screen: SelectedScreen::Loading,
            search_buffer: String::new(),
            search_results: Vec::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel(ui);
        });
        egui::SidePanel::right("scen_switch").show(ctx, |ui| {
            self.right_panel(ui);
        });
        match self.update_data() {
            Ok(()) => (),
            Err(e) => self.add_err_popup(format!("Could not update data: {}", e)),
        }
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
