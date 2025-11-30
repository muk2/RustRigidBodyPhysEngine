use crate::types::Vec2;

/// Semi-implicit (a.k.a. symplectic) Euler integration
/// Updates velocity then position. Resets forces to zero.
pub fn semi_implicit_euler(position: &mut Vec2, velocity: &mut Vec2, forces: &mut Vec2, mass: f32, dt:f32){
    // acceleration = forces / mass
    let acc = forces.div(mass);
    // v_{t+dt} = v_t + a * dt
    *velocity = velocity.add(acc.scale(dt));
    // x_{t+dt} = x_t + v_{t+dt} * dt
    *position = position.add(acc.scale(dt));

    //test forces zero
    *forces = Vec2::zero();

}

/// Simple explicit Euler (kept for reference)
pub fn explicit_euler(pos: &mut Vec2, vel: &mut Vec2, forces: &mut Vec2, mass: f32, dt: f32) {
    let acc = forces.div(mass);
    *pos = pos.add(vel.scale(dt));
    *vel = vel.add(acc.scale(dt));
    *forces = Vec2::zero();
}


