use plotters::prelude::*;
use crate::{GraphType, ScenarioState};
use crate::reader::{GeneralData, ScenarioData};

pub fn get_scen_search_results(general_data: &GeneralData, query: &str) -> Vec<String> {
    general_data.scenarios
        .keys()
        .filter(|key| key.to_lowercase().contains(query.to_lowercase().as_str()))
        .map(|key| key.clone())
        .take(25)
        .collect::<Vec<String>>()
}
// pub fn get_scen_search_results<'a>(general_data: &'a GeneralData, query: &'a str) -> impl Iterator<Item = &'a String> {
//     general_data.scenarios.keys().filter(move |key| key.contains(query)).take(5)
// }

pub fn generate_plot(scenario_state: &ScenarioState, data: &ScenarioData) {
    println!("Creating plot for {}", scenario_state.name);

    // let root = BitMapBackend::new(&scenario_state.name, (1024, 769)).into_drawing_area();

    // root.fill(&WHITE).unwrap();

    // let areas = root.split_by_breakpoints([944], [800]);

    // let mut x_hist_ctx = ChartBuilder::on(&areas[0])
    //     .y_label_area_size(40)
    //     .build_cartesian_2d((0.0..1.0).step(0.01).use_round().into_segmented(), 0..250).unwrap();
    // let mut y_hist_ctx = ChartBuilder::on(&areas[3])
    //     .x_label_area_size(40)
    //     .build_cartesian_2d(0..250, (0.0..1.0).step(0.01).use_round()).unwrap();
    // let mut scatter_ctx = ChartBuilder::on(&areas[2])
    //     .x_label_area_size(40)
    //     .y_label_area_size(40)
    //     .build_cartesian_2d(0f64..1f64, 0f64..1f64).unwrap();
    // scatter_ctx
    //     .configure_mesh()
    //     .disable_x_mesh()
    //     .disable_y_mesh()
    //     .draw().unwrap();
    // scatter_ctx.draw_series()

}
