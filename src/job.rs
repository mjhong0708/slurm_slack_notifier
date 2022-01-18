use std::process::Command;
use std::str::FromStr;

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
            .expect("Failed to execute process");
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
            .expect("Failed to execute process");
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

#[test]
fn test_slack() {
    use crate::config;
    use slack_hook::PayloadBuilder;
    use slack_hook::Slack;
    let config = config::Config::new();
    let slack = Slack::new(&config.slack.endpoint[..]).unwrap();
    let p = PayloadBuilder::new()
        .text("test message")
        .channel(&config.slack.channel)
        .username(&config.slack.app_name)
        .icon_emoji(":chart_with_upwards_trend:")
        .build()
        .expect("failed to build payload");
    slack.send(&p).expect("Failed to send message");
}
