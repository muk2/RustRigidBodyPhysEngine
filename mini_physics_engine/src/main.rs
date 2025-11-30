// src/main.rs

mod types;
mod integrator;
mod body;

use crate::types::Vec2;
use crate::body::{Body, World};

fn main() {
    // create a world with downward gravity (m/s^2)
    let mut world = World::new(Vec2::new(0.0, -9.81));

    // create a single body (position y=10)
    let b = Body::new(Vec2::new(0.0, 10.0), Vec2::new(0.0, 0.0), 1.0);
    world.add_body(b);

    // simulate for 200 frames at dt = 0.016s (~60Hz)
    let dt = 0.016;
    for _ in 0..200 {
        world.step(dt);
        // print position of first body
        if let Some(first) = world.bodies.get(0) {
            println!("{:?}", first.position);
        }
    }
}