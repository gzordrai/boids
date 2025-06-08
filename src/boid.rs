use macroquad::{color::RED, math::Vec2, rand::gen_range, shapes::draw_circle};

use crate::CONFIG;

#[derive(Clone)]
pub struct Boid {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
}

impl Boid {
    pub fn new(radius: f32) -> Self {
        let width = CONFIG.window_width as f32;
        let height = CONFIG.window_height as f32;

        Boid {
            position: Vec2::new(
                gen_range(radius, width - radius),
                gen_range(radius, height - radius),
            ),
            velocity: Vec2::new(gen_range(-2.0, 2.0), gen_range(-2.0, 2.0)),
            radius,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, RED);
    }

    pub fn update(&mut self, boids: &Vec<Boid>) {
        self.update_position_with_bounds();
        self.calculate_separation_force(boids);
    }

    fn update_position_with_bounds(&mut self) {
        let height = CONFIG.window_height as f32;
        let width = CONFIG.window_width as f32;
        let position = self.position + self.velocity;

        // Left and right walls
        if position.x < self.radius || position.x > width - self.radius {
            self.velocity.x = -self.velocity.x;
            self.velocity.y += gen_range(-1.0, 1.0);
        }

        // Top and bottom walls
        if position.y < self.radius || position.y > height - self.radius {
            self.velocity.y = -self.velocity.y;
            self.velocity.x += gen_range(-1.0, 1.0);
        }

        self.position += self.velocity
    }

    fn calculate_separation_force(&mut self, boids: &Vec<Boid>) {
        let mut steer = Vec2::ZERO;
        let mut count = 0;

        for boid in boids {
            let diff = self.position - boid.position;
            let distance = diff.length();

            if distance > 0.0 && distance < self.radius {
                steer += diff.normalize() / distance;
                count += 1;
            }
        }

        if count > 0 {
            steer /= count as f32;

            if steer.length() > 0.0 {
                steer = steer.normalize() * 0.5;
            }

            self.velocity += steer;
        }
    }
}
