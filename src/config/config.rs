use std::fs::read_to_string;

use serde::Deserialize;
use toml::from_str;

#[derive(Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    pub boids: BoidsConfig,
}

#[derive(Deserialize)]
pub struct WindowConfig {
    pub window_title: String,
    pub window_height: i32,
    pub window_width: i32,
}

#[derive(Deserialize)]
pub struct BoidsConfig {
    pub number: i32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub neighborhood_radius: f32,
    pub separation_radius: f32,
    pub separation_strength: f32,
    pub alignment_strength: f32,
    pub cohesion_strength: f32,
    pub debug: bool,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let config = match read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Config::default(),
        };

        match from_str::<Config>(&config) {
            Ok(config) => config,
            Err(_) => Config::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            window: WindowConfig {
                window_title: "Boids simulation".to_owned(),
                window_height: 600,
                window_width: 800,
            },
            boids: BoidsConfig {
                number: 100,
                min_speed: 2.0,
                max_speed: 5.0,
                max_force: 0.05,
                neighborhood_radius: 60.0,
                separation_radius: 40.0,
                separation_strength: 3.0,
                alignment_strength: 1.8,
                cohesion_strength: 0.5,
                debug: false,
            },
        }
    }
}
