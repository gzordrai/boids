use macroquad::{color::RED, math::Vec2, shapes::draw_circle};

pub struct Boid {
    position: Vec2,
    velocity: Vec2,
}

impl Boid {
    pub fn new() -> Self {
        Boid {
            position: Vec2::new(10.0, 10.0),
            velocity: Vec2::new(1.0, 1.0),
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 6.0, RED);
    }

    pub fn update(&mut self) {
        self.position += self.velocity
    }
}
