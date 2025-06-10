use std::f32::consts::PI;
use macroquad::{
    color::{GRAY, RED},
    math::Vec2,
    rand::gen_range,
    shapes::{draw_circle_lines, draw_triangle},
};

use crate::CONFIG;

#[derive(Clone)]
pub struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    radius: f32,
}

impl Boid {
    pub fn new(radius: f32) -> Self {
        let width = CONFIG.window.window_width as f32;
        let height = CONFIG.window.window_height as f32;

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
        let angle = self.velocity.y.atan2(self.velocity.x);
        let tip = self.position + Vec2::new(angle.cos(), angle.sin()) * self.radius * 2.0;
        let left = self.position
            + Vec2::new((angle + PI * 0.75).cos(), (angle + PI * 0.75).sin()) * self.radius;
        let right = self.position
            + Vec2::new((angle - PI * 0.75).cos(), (angle - PI * 0.75).sin()) * self.radius;

        draw_triangle(tip, left, right, RED);

        if CONFIG.boids.debug {
            draw_circle_lines(
                self.position.x,
                self.position.y,
                CONFIG.boids.separation_radius,
                1.0,
                GRAY,
            );
        }
    }

    pub fn update(&mut self, boids: &Vec<Boid>) {
        let separation = self.calculate_separation_force(boids) * CONFIG.boids.separation_strength;
        let alignment = self.calculate_alignment_force(boids) * CONFIG.boids.alignment_strength;
        let cohesion = self.calculate_cohesion_force(boids) * CONFIG.boids.cohesion_strength;
        let mut steering = separation + alignment + cohesion;

        if steering.length() > CONFIG.boids.max_force {
            steering = steering.normalize() * CONFIG.boids.max_force;
        }

        self.acceleration += steering;
        self.velocity = self.velocity.lerp(self.velocity + self.acceleration, 0.2);

        if self.velocity.length() > CONFIG.boids.max_speed {
            self.velocity = self.velocity.normalize() * CONFIG.boids.max_speed;
        }

        self.update_position_with_bounds();
        self.acceleration = Vec2::ZERO;
    }

    fn update_position_with_bounds(&mut self) {
        let height = CONFIG.window.window_height as f32;
        let width = CONFIG.window.window_width as f32;
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

            if distance > 0.0 && distance < CONFIG.boids.separation_radius {
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

            if distance > 0.0 && distance < CONFIG.boids.neighborhood_radius {
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

            if distance > 0.0 && distance < CONFIG.boids.neighborhood_radius {
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
