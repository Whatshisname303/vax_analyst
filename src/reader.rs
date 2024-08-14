use std::error::Error;
use std::fs;
use std::collections::HashMap;

pub struct GeneralData {
    pub scenario_plays: u32,
    pub scenarios: HashMap<String, ScenarioData>,
}

pub struct ScenarioData {
    pub plays: Vec<ScenarioRun>,
}

pub struct ScenarioRun {
    pub score: f32,
    pub timestamp: u32,
}

pub fn get_general_data() -> Result<GeneralData, Box<dyn Error>> {
    let mut plays = 0;
    let mut scenarios: HashMap<String, ScenarioData> = HashMap::new();

    for file in fs::read_dir("stats")? {
        let file = file?;
        let path = file.path();
        let name = match path.file_name() {
            Some(name) => name.to_str().unwrap(),
            None => continue,
        };
        let name = get_scenario_name(name).unwrap_or("Unknown name".to_string());

        match scenarios.get_mut(&name) {
            Some(scen) => {
                scen.plays.push(read_scenario_run(&path)?);
            },
            None => {
                scenarios.insert(name, ScenarioData {
                    plays: vec![read_scenario_run(&path)?]
                });
            },
        };

        plays += 1;
    }

    Ok(GeneralData {
        scenario_plays: plays,
        scenarios,
    })
}

fn get_scenario_name(path: &str) -> Option<String> {
    let challenge_index = path.find(" - Challenge - ")?;
    let name = path[0..challenge_index].to_string();
    Some(name)
}

fn read_scenario_run(path: &std::path::PathBuf) -> Result<ScenarioRun, Box<dyn Error>> {
    let data = fs::read(path)?;
    let utf8 = String::from_utf8(data)?;

    let mut score_index = utf8.find("Score:").ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Score field not found"))?;
    score_index += 7; // Skip "Score:,"
    let mut end_index = score_index;

    for byte in utf8[score_index..utf8.len()].bytes() {
        if byte == b'\r' || byte == b'\n' {
            break;
        }
        end_index += 1;
    }

    Ok(ScenarioRun {
        score: utf8[score_index..end_index].parse()?,
        timestamp: 8,
    })
}
