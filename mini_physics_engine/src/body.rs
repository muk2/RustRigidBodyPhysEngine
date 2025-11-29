use crate::types::Vec2;

pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub forces: Vec2,
    pub mass: f32,
}

pub struct World {
    pub bodies: Vec<Body>,
    pub gravity: Vec2,
}