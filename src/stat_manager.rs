use core::f32;

use plotters::prelude::*;
use crate::{ScenarioState, SearchSort};
use crate::reader::ScenarioData;

pub fn get_scen_search_results(app: &crate::App) -> Vec<String> {
    let scens = &app.general_data.as_ref().unwrap().scenarios;
    let query = &app.search_buffer;
    let sort = &app.search_sort;

    let mut scenario_names = scens
        .keys()
        .filter(|key| key.to_lowercase().contains(query.to_lowercase().as_str())).cloned()
        .collect::<Vec<String>>();

    let compare: Box<dyn Fn(&String, &String) -> std::cmp::Ordering> = match sort {
        SearchSort::None => Box::new(|_a, _b| std::cmp::Ordering::Equal),
        SearchSort::Plays => Box::new(|a, b| scens.get(b).unwrap().plays.len().cmp(&scens.get(a).unwrap().plays.len())),
    };

    scenario_names.sort_by(compare);
    scenario_names.truncate(app.config.num_search_results as usize);
    scenario_names
}

fn get_plot_bounds(scenario_data: &ScenarioData) -> ((u64, u64), (f32, f32)) {
    let mut score_min = f32::MAX;
    let mut score_max = 0f32;
    let mut time_min = u64::MAX;
    let mut time_max = 0u64;

    for run in &scenario_data.plays {
        if run.score < score_min {
            score_min = run.score;
        } else if run.score > score_max {
            score_max = run.score;
        }
        if run.timestamp < time_min {
            time_min = run.timestamp;
        } else if run.timestamp > time_max {
            time_max = run.timestamp
        }
    }

    score_max += (score_max - score_min) * 0.1;

    ((time_min, time_max), (score_min, score_max))
}

pub fn generate_plot(scenario_state: &ScenarioState, data: &ScenarioData) -> Result<String, Box<dyn std::error::Error>> {
    println!("Creating plot for {}", scenario_state.name);

    let white_color = &RGBColor(167, 167, 167);
    let white_faded = &RGBColor(99, 99, 99);
    let white_mist = &RGBColor(70, 70, 70);
    let ((x_min, x_max), (y_min, y_max)) = get_plot_bounds(data);
    let plot_path = format!("plots/{}-plot.png", scenario_state.name);

    let root = BitMapBackend::new(&plot_path, (640, 480)).into_drawing_area();
    root.fill(&RGBColor(54, 54, 54))?;
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{} - Score/Time", scenario_state.name), ("sans-serif", 40).into_font().color(white_color))
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .x_label_formatter(&|x| chrono::DateTime::from_timestamp(*x as i64, 0).unwrap().format("%Y-%m-%d").to_string())
        .axis_style(ShapeStyle::from(white_color))
        .label_style(("sans-serif", 14, white_color))
        .bold_line_style(ShapeStyle::from(white_faded).stroke_width(1))
        .light_line_style(ShapeStyle::from(white_mist))
        .draw()?;

    chart.draw_series(PointSeries::of_element(
        data.plays.iter().map(|run| (run.timestamp, run.score)).collect::<Vec<(u64, f32)>>(),
        2,
        &RED,
        &|cord, size, style| {
            EmptyElement::at(cord)
                + Circle::new((0,0), size, style.filled())
                // + Text::new(format!("{:?}", cord), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;

    root.present()?;
    Ok(plot_path.clone())
}
