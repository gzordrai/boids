use macroquad::prelude::*;
use once_cell::sync::Lazy;

use crate::{boids::Boids, config::config::Config};

mod boid;
mod boids;
mod config;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new("config.toml"));

fn window_conf() -> Conf {
    Conf {
        window_title: CONFIG.window.window_title.clone(),
        window_height: CONFIG.window.window_height,
        window_width: CONFIG.window.window_width,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut boids = Boids::new(CONFIG.boids.number, 6.0);

    loop {
        clear_background(BLACK);

        boids.update();
        boids.draw();

        next_frame().await
    }
}
