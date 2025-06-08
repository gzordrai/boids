use std::fs::read_to_string;

use serde::Deserialize;
use toml::from_str;

#[derive(Deserialize)]
pub struct Config {
    pub window_title: String,
    pub window_height: i32,
    pub window_width: i32,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let config = match read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Config::default()
        };

        match from_str::<Config>(&config) {
            Ok(config) => config,
            Err(_) => Config::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            window_title: "Boids simulation".to_owned(),
            window_height: 600,
            window_width: 800
        }
    }
}
