use super::super::planet::Planet;
use super::super::random::DspRandom;
use super::super::random_table::RandomTable;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;
use crate::data::planet_grid::{get_planet_grid, PlanetGrid};

/// PlanetAlgorithm4 - FBM noise with 80 circular crater features.
pub struct PlanetAlgorithm4 {
    grid: &'static PlanetGrid,
    radius: f64,
    noise1: SimplexNoise,
    noise2: SimplexNoise,
    circles: Vec<([f64; 3], f64)>,
    heights: Vec<f64>,
}

impl PlanetAlgorithm4 {
    pub fn new(planet: &Planet) -> Self {
        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();

        let mut seed3 = rand.next_seed();
        let mut cr = Vec::with_capacity(80);
        let mut hs = Vec::with_capacity(80);

        for _ in 0..80 {
            let mut v = RandomTable::spheric_normal(&mut seed3, 1.0);
            let w = (v.magnitude() * 8.0 + 8.0) as f32;
            v.normalize();
            v *= planet.radius as f64;
            cr.push(([v.0, v.1, v.2], (w * w) as f64));

            let h = rand.next_f64() * 0.4 + 0.2;
            hs.push(h);
        }

        Self {
            grid: get_planet_grid(),
            radius: planet.radius as f64,
            noise1: SimplexNoise::with_seed(seed1),
            noise2: SimplexNoise::with_seed(seed2),
            circles: cr,
            heights: hs,
        }
    }
}

impl PlanetAlgorithm for PlanetAlgorithm4 {
    fn get_height(&self, index: usize) -> f64 {
        let freq_scale_x: f64 = 0.007;
        let freq_scale_y: f64 = 0.007;
        let freq_scale_z: f64 = 0.007;

        let v = self.grid.get_vertex(index);
        let world_x = (v.0 as f64) * self.radius;
        let world_y = (v.1 as f64) * self.radius;
        let world_z = (v.2 as f64) * self.radius;

        let low_freq_noise = self.noise1.noise_3d_fbm(
            world_x * freq_scale_x,
            world_y * freq_scale_y,
            world_z * freq_scale_z,
            4,
            0.45,
            1.8,
        );
        let high_freq_noise = self.noise2.noise_3d_fbm(
            world_x * freq_scale_x * 5.0,
            world_y * freq_scale_y * 5.0,
            world_z * freq_scale_z * 5.0,
            4,
            0.5,
            2.0,
        );

        let scaled_low = low_freq_noise * 1.5;
        let scaled_high = high_freq_noise * 0.2;
        let base_elevation = scaled_low * 0.08 + scaled_high * 2.0;

        let mut max_crater = 0.0;
        for j in 0..80 {
            let c = &self.circles[j];
            let dx = c.0[0] - world_x;
            let dy = c.0[1] - world_y;
            let dz = c.0[2] - world_z;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            if dist_sq <= c.1 {
                let mut t = dist_sq / c.1 + scaled_high * 1.2;
                if t < 0.0 {
                    t = 0.0;
                }
                let t_sq = t * t;
                let crater_shape = -15.0 * (t_sq * t) + (131.0 / 6.0) * t_sq - (113.0 / 15.0) * t
                    + 0.7
                    + scaled_high;
                let crater_shape = if crater_shape < 0.0 {
                    0.0
                } else {
                    crater_shape
                };
                let crater_val = crater_shape * crater_shape * self.heights[j];
                if crater_val > max_crater {
                    max_crater = crater_val;
                }
            }
        }

        let final_height = max_crater + base_elevation + 0.2;
        self.radius + final_height + 0.1
    }
}
