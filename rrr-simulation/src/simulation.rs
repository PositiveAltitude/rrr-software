use bevy::math::{Quat, Vec3};

struct Rocket {
    mass: f32,
    moments_of_inertia: Vec3,
    thrust: f32
}

struct SimulationConfig {
    time_end: f32,
    dt: f32,
    g: f32,
    wind: Vec3,
    sub_steps: u32
}

struct RocketState {
    time: f32,
    position: Vec3,
    velocity: Vec3,
    orientation: Quat
}

fn simulate(rocket: Rocket, simulation_config: SimulationConfig , initial_state: &mut RocketState) -> Vec<RocketState> {
    Vec::new()
}