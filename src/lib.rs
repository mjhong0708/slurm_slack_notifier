use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub slack: SlackConfig,
}

#[derive(Serialize, Deserialize)]
pub struct SlackConfig {
    pub endpoint: String,
    pub app_name: String,
    pub channel: String,
}

impl Config {
    pub fn new() -> Config {
        let config_path = {
            let mut home_path = home::home_dir().unwrap();
            home_path.push(".config/monitorjob/config.json");
            home_path
        };
        let config_file = fs::read_to_string(config_path);
        match config_file {
            Ok(contents) => {
                let config_data: Config =
                    serde_json::from_str(&contents).expect("Error in parsing json");
                config_data
            }
            Err(msg) => {
                panic!("{}", msg);
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum JobState {
    Queued,
    Runnning,
    Complete,
}

impl FromStr for JobState {
    type Err = ();
    fn from_str(state: &str) -> Result<JobState, ()> {
        match state {
            "Q" => Ok(JobState::Queued),
            "R" => Ok(JobState::Runnning),
            "C" => Ok(JobState::Complete),
            _ => {
                panic!("Failed parsing job state");
            }
        }
    }
}

#[derive(Debug)]
pub struct Job {
    pub id: i32,
    pub user: String,
    pub partition: String,
    pub name: String,
    pub n_nodes: i32,
    pub n_tasks: i32,
    pub state: JobState,
    pub elapsed: String,
}

impl Job {
    pub fn from_line(line: String) -> Job {
        let components = line.split_whitespace().collect::<Vec<&str>>();

        let id: i32 = components[0].parse().expect("Fail");
        let user = components[1].to_string();
        let partition = components[2].to_string();
        let name = components[3].to_string();
        let n_nodes: i32 = components[5].parse().expect("Fail");
        let n_tasks: i32 = components[6].parse().expect("Fail");
        let state = JobState::from_str(components[9]).unwrap();
        let elapsed = components[10].to_string();
        Job {
            id,
            user,
            partition,
            name,
            n_nodes,
            n_tasks,
            state,
            elapsed,
        }
    }
}

pub fn get_qstat_lines(user: &str) -> Vec<String> {
    let qstat = {
        let output = Command::new("qstat")
            .arg("-u")
            .arg(user)
            .output()
            .expect("failed to execute process");
        String::from_utf8(output.stdout).expect("Failed to convert to string")
    };
    let lines: Vec<String> = qstat
        .trim()
        .lines()
        .skip(4)
        .map(|c| c.to_string())
        .collect();

    lines
}

pub fn get_job_dir(id: i32) -> String {
    let jobinfo = {
        let output = Command::new("scontrol")
            .arg("show")
            .arg("job")
            .arg(format!("{}", id))
            .output()
            .expect("failed to execute process");
        String::from_utf8(output.stdout).expect("Failed to convert to string")
    };
    let workdir = jobinfo
        .lines()
        .filter(|s| s.trim().starts_with("WorkDir"))
        .collect::<Vec<&str>>()[0]
        .trim();

    let workdir = workdir.to_string();
    let workdir = &workdir[8..];
    workdir.to_string()
}
