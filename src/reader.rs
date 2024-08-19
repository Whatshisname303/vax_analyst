use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs;
use std::collections::HashMap;
use std::time::SystemTime;

pub struct GeneralData {
    pub scenario_plays: u32,
    pub scenarios: HashMap<String, ScenarioData>,
}

pub struct ScenarioData {
    pub plays: Vec<ScenarioRun>,
}

pub struct ScenarioRun {
    pub score: f32,
    pub timestamp: u64,
}

pub fn get_general_data() -> Result<GeneralData, Box<dyn Error>> {
    let mut plays = 0;
    let mut scenarios: HashMap<String, ScenarioData> = HashMap::new();

    for file in fs::read_dir("garbage/my_stats")? {
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
    let file = fs::File::open(path)?;

    let timestamp = file.metadata()?.created()?.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let mut score: Option<f32> = None;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() > 6 && &line[..6] == "Score:" {
            score = Some(line[7..].parse()?);
        }
    }

    match score {
        Some(score) => Ok(ScenarioRun {
            score,
            timestamp
        }),
        None => Err("Could not read score from csv".into()),
    }
}
