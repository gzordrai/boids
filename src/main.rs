use macroquad::prelude::*;

use crate::boids::Boids;

mod boid;
mod boids;

fn window_conf() -> Conf {
    Conf {
        window_title: "Boids simulation".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut boids = Boids::new(10);

    loop {
        clear_background(BLACK);

        boids.update();
        boids.draw();

        next_frame().await
    }
}
