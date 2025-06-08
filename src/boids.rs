use crate::boid::Boid;

pub struct Boids {
    boids: Vec<Boid>,
}

impl Boids {
    pub fn new(n: i32, radius: f32) -> Self {
        let mut boids: Vec<Boid> = Vec::new();

        for _ in 0..n {
            boids.push(Boid::new(radius));
        }

        Boids { boids }
    }

    pub fn draw(&self) {
        for boid in &self.boids {
            boid.draw();
        }
    }

    pub fn update(&mut self) {
        for boid in &mut self.boids {
            boid.update();
        }
    }
}
