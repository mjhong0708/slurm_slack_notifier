use std::collections::HashMap;

extern crate slack_hook;
use monitorjob::{get_job_dir, get_qstat_lines, Config, Job, JobState, SlackConfig};
use slack_hook::{PayloadBuilder, Slack};
use std::time::Duration;

fn main() {
    let config = Config::new();

    // Initialize
    let lines = get_qstat_lines(&config.username);
    let mut job_table: HashMap<i32, Job> = HashMap::new();
    for line in lines {
        let job = Job::from_line(line);
        match job.state {
            JobState::Complete => (),
            _ => {
                job_table.insert(job.id, job);
            }
        }
    }

    loop {
        let lines = get_qstat_lines(&config.username);
        for line in lines {
            let curr_job = Job::from_line(line);
            if job_table.contains_key(&curr_job.id) {
                let prev_job = job_table.get(&curr_job.id).unwrap();
                match prev_job.state {
                    JobState::Complete => {
                        let msg = format!(
                            "Completed job {}\nName: {}\nWork dir: {}\nElapsed time: {}",
                            &curr_job.id,
                            &curr_job.name,
                            get_job_dir(prev_job.id),
                            &curr_job.elapsed
                        );
                        println!("{}", &msg);
                        job_table.remove(&curr_job.id);
                        send_slack(&config.slack, &msg);
                    }
                    _ => {
                        job_table.insert(curr_job.id, curr_job);
                    }
                }
            } else {
                match curr_job.state {
                    JobState::Complete => (),
                    _ => {
                        job_table.insert(curr_job.id, curr_job);
                    }
                }
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn send_slack(slack_config: &SlackConfig, msg: &str) {
    let url = &slack_config.endpoint;
    let slack = Slack::new(&url[..]).unwrap();

    let p = PayloadBuilder::new()
        .text(msg.to_string())
        .channel(&slack_config.channel)
        .username(&slack_config.app_name)
        .icon_emoji(":chart_with_upwards_trend:")
        .build()
        .unwrap();
    slack.send(&p).expect("Failed to send message");
}
