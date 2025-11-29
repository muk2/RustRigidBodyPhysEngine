use crate::types::Vec2;
use crate::body::{Body, World};

pub fn step(b: &mut Body, dt: f32) {
    let acc = Vec2::new(b.forces.x / b.mass, b.forces.y / b.mass);
    b.velocity = b.velocity.add(acc.scale(dt));
    b.position = b.position.add(b.velocity.scale(dt));
    b.forces = Vec2::new(0.0, 0.0); // reset
}

