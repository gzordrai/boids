use macroquad::{color::RED, math::Vec2, rand::gen_range, shapes::draw_circle};

use crate::CONFIG;

pub struct Boid {
    position: Vec2,
    velocity: Vec2,
    radius: f32
}

impl Boid {
    pub fn new(radius: f32) -> Self {
        Boid {
            position: Vec2::new(10.0, 10.0),
            velocity: Vec2::new(1.0, 1.0),
            radius
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, RED);
    }

    pub fn update(&mut self) {
        self.update_position_with_bounds();
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
}
