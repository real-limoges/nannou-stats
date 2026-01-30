use macroquad::prelude::*;
use macroquad::rand::gen_range;

#[derive(Debug, Clone)]
pub struct Dataset {
    pub points: Vec<Vec3>,
}

impl Dataset {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn from_points(points: Vec<Vec3>) -> Self {
        Self { points }
    }

    /// Generate sample GAM style data . Creates a wavy surface with noise
    pub fn generate_gam_data() -> Self {
        let mut points = Vec::new();
        let n_points = 50;

        for _ in 0..n_points {
            let x: f32 = gen_range(-2.5, 2.5);
            let y: f32 = gen_range(-2.5, 2.5);

            let z_base = x.sin() + (y / 1.2).cos() + 0.5;
            let z_noisy = z_base + gen_range(-0.3, 0.3);

            points.push(vec3(x, y, z_noisy));
        }
        Self { points }
    }

    /// Generate linear regression sample data
    pub fn generate_linear_data(n_points: usize, slope: f32, intercept: f32, noise: f32) -> Self {
        let points = (0..n_points)
            .map(|_| {
                let x = gen_range(-3.0, 3.0);
                let y = slope * x + intercept + gen_range(-noise, noise);
                vec3(x, y, 0.0)
            })
            .collect();
        Self { points }
    }

    /// Generate clustered sample data
    pub fn generate_clusters(n_clusters: usize, points_per_cluster: usize, spread: f32) -> Self {
        let mut points = Vec::new();

        for i in 0..n_clusters {
            let angle = (i as f32 / n_clusters as f32) * std::f32::consts::TAU;
            let center = vec2(angle.cos() * 2.0, angle.sin() * 2.0);

            for _ in 0..points_per_cluster {
                let offset = vec2(gen_range(-spread, spread), gen_range(-spread, spread));
                let p = center + offset;
                points.push(vec3(p.x, p.y, i as f32)); // z stores cluster ID
            }
        }
        Self { points }
    }

    /// Get 2D projection (drop z coordinate)
    pub fn as_2d(&self) -> Vec<Vec2> {
        self.points.iter().map(|p| vec2(p.x, p.y)).collect()
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

impl Default for Dataset {
    fn default() -> Self {
        Self::new()
    }
}
