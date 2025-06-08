use macroquad::{color::RED, math::Vec2, rand::gen_range, shapes::draw_circle};

use crate::CONFIG;

const MAX_SPEED: f32 = 5.0;
const MAX_FORCE: f32 = 0.05;

const NEIGHBORHOOD_RADIUS: f32 = 60.0;
const SEPARATION_RADIUS: f32 = 20.0;

const SEPARATION_STRENGTH: f32 = 3.0;
const ALIGNMENT_STRENGTH: f32 = 1.8;
const COHESION_STRENGTH: f32 = 1.0;

#[derive(Clone)]
pub struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
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
            acceleration: Vec2::ZERO,
            radius,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, RED);
    }

    pub fn update(&mut self, boids: &Vec<Boid>) {
        let separation = self.calculate_separation_force(boids) * SEPARATION_STRENGTH;
        let alignment = self.calculate_alignment_force(boids) * ALIGNMENT_STRENGTH;
        let cohesion = self.calculate_cohesion_force(boids) * COHESION_STRENGTH;
        let mut steering = separation + alignment + cohesion;

        if steering.length() > MAX_FORCE {
            steering = steering.normalize() * MAX_FORCE;
        }

        self.acceleration += steering;
        self.velocity = self.velocity.lerp(self.velocity + self.acceleration, 0.2);

        if self.velocity.length() > MAX_SPEED {
            self.velocity = self.velocity.normalize() * MAX_SPEED;
        }

        self.update_position_with_bounds();
        self.acceleration = Vec2::ZERO;
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

    fn calculate_separation_force(&self, boids: &Vec<Boid>) -> Vec2 {
        let mut steer = Vec2::ZERO;
        let mut count = 0;

        for boid in boids {
            let diff = self.position - boid.position;
            let distance = diff.length();

            if distance > 0.0 && distance < SEPARATION_RADIUS {
                steer += diff.normalize() / distance;
                count += 1;
            }
        }

        if count > 0 {
            steer /= count as f32;

            if steer.length() > 0.0 {
                steer = steer.normalize();
            }
        }

        steer
    }

    fn calculate_alignment_force(&self, boids: &Vec<Boid>) -> Vec2 {
        let mut avg_velocity = Vec2::ZERO;
        let mut count = 0;

        for boid in boids {
            let distance = (self.position - boid.position).length();

            if distance > 0.0 && distance < NEIGHBORHOOD_RADIUS {
                avg_velocity += boid.velocity;
                count += 1;
            }
        }

        if count > 0 {
            avg_velocity /= count as f32;
            let mut steer = avg_velocity - self.velocity;

            if steer.length() > 0.0 {
                steer = steer.normalize();

                return steer;
            }
        }

        Vec2::ZERO
    }

    fn calculate_cohesion_force(&self, boids: &Vec<Boid>) -> Vec2 {
        let mut center_of_mass = Vec2::ZERO;
        let mut count = 0;

        for boid in boids {
            let distance = (self.position - boid.position).length();

            if distance > 0.0 && distance < NEIGHBORHOOD_RADIUS {
                center_of_mass += boid.position;
                count += 1;
            }
        }

        if count > 0 {
            center_of_mass /= count as f32;
            let mut steer = center_of_mass - self.position;

            if steer.length() > 0.0 {
                steer = steer.normalize();
                return steer;
            }
        }

        Vec2::ZERO
    }
}
