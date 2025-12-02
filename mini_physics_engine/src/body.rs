use crate::types::Vec2;
use crate::integrator;

pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub forces: Vec2,
    pub mass: f32,
}


impl Body {
    pub fn new(position: Vec2, velocity: Vec2, mass: f32) -> Self {
        Self {
            position,
            velocity,
            forces: Vec2::zero(),
            mass,
        }
    }

    /// Add a force to the accumulated force on this body
    pub fn apply_force(&mut self, f: Vec2) {
        self.forces = self.forces.add(f);
    }

    /// Integrate this body's state forward by dt using semi-implicit Euler
    pub fn integrate(&mut self, dt: f32) {
        integrator::semi_implicit_euler(&mut self.position, &mut self.velocity, &mut self.forces, self.mass, dt);
    }
}

pub struct World {
    pub bodies: Vec<Body>,
    pub gravity: Vec2,
}

impl World {
    pub fn new(gravity: Vec2) -> Self {
        Self { bodies: Vec::new(), gravity }
    }

    pub fn add_body(&mut self, b: Body) {
        self.bodies.push(b);
    }

    /// Step the entire world forward by dt:
    /// - apply gravity to each body (force = gravity * mass)
    /// - integrate each body (which will reset forces)
    pub fn step(&mut self, dt: f32) {
        for body in &mut self.bodies {
            // gravity force = gravity_acceleration * mass
            let g_force = self.gravity.scale(body.mass);
            body.apply_force(g_force);
            body.integrate(dt);
            // note: body.integrate resets forces for next frame

            let ground_y = 0.0;
            if body.position.y < ground_y{
                body.position.y = ground_y;

                body.velocity.y *= -0.5;
            }
        }
    }
}
