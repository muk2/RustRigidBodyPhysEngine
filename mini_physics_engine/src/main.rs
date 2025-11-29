mod types;
mod body;
mod integrator;
use crate::body::Body;
use crate::integrator::step;
use crate::types::Vec2;

fn main() {
    let mut body = Body {
        position: Vec2::new(0.0, 10.0),
        velocity: Vec2::new(0.0, 0.0),
        forces: Vec2::new(0.0, -9.81),
        mass: 1.0,
    };

    for _ in 0..200 {
        step(&mut body, 0.016);
        println!("{:?}", body.position);
    }
}