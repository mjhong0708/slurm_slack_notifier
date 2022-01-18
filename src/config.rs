use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub slack: SlackConfig,
}

#[derive(Debug, Serialize, Deserialize)]
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
