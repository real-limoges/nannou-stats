use nannou::prelude::*;

pub struct SimulationState {
    pub complexity: f32,  // this can go from 0.0 to 2.0
    pub rotation_angle: f32,
}

impl SimulationState {
    pub fn new() -> Self {
        Self {
            complexity: 0.0,
            rotation_angle: 0.0,
        }
    }
}

pub struct Dataset {
    pub points: Vec<Vec3>,
}

impl Dataset {
    pub fn generate_gam_data() -> Self {
        let mut points = Vec::new();
        let n_points = 50;

        for _ in 0..n_points {
            let x = random_range(-2.5, 2.5);
            let y = random_range(-2.5, 2.5);

            let z_base = x.sin() + (y / 1.2).cos() + 0.5;
            let z_noisy = z_base + random_range(-0.3, 0.3);

            points.push(vec3(x, y, z_noisy));
        }
        Self { points }
    }
}