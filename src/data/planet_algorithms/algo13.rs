use super::super::planet::Planet;
use crate::data::planet_grid::{get_planet_grid, PlanetGrid};
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm13 - Noise-based terrain with modX/modY and piecewise height shaping.
pub struct PlanetAlgorithm13 {
    grid: &'static PlanetGrid,
    radius: f64,
    mod_y: f64,
    noise: SimplexNoise,
    freq_scale_x: f64,
    freq_scale_y: f64,
    freq_scale_z: f64,
}

#[inline]
fn remap(src_min: f64, src_max: f64, tgt_min: f64, tgt_max: f64, x: f64) -> f64 {
    (x - src_min) / (src_max - src_min) * (tgt_max - tgt_min) + tgt_min
}

impl PlanetAlgorithm13 {
    pub fn new(planet: &Planet) -> Self {
        let mod_x = planet.get_mod_x();
        Self {
            grid: get_planet_grid(),
            radius: planet.radius as f64,
            mod_y: planet.get_mod_y(),
            noise: SimplexNoise::with_seed(DspRandom::new(planet.seed).next_seed()),
            freq_scale_x: 0.007 * mod_x,
            freq_scale_y: 0.007 * mod_x,
            freq_scale_z: 0.007 * mod_x,
        }
    }
}

impl PlanetAlgorithm for PlanetAlgorithm13 {
    fn get_height(&self, index: usize) -> f64 {
        let v = self.grid.get_vertex(index);
        let world_x = (v.0 as f64) * self.radius;
        let world_y = (v.1 as f64) * self.radius;
        let world_z = (v.2 as f64) * self.radius;

        let n = self.noise.noise_3d_fbm(
            world_x * self.freq_scale_x,
            world_y * self.freq_scale_y,
            world_z * self.freq_scale_z,
            6,
            0.5,
            2.0,
        );
        let mut raw_height = remap(
            0.0,
            2.0,
            0.0,
            4.0,
            remap(-1.0, 1.0, 0.0, 1.0, n).powf(self.mod_y) * (49.0 / 16.0),
        );

        if raw_height < 1.0 {
            raw_height = raw_height.powi(2);
        }

        let clamped_height = (raw_height - 0.2).min(4.0);

        let final_height = if clamped_height > 2.0 {
            if clamped_height <= 3.0 {
                2.0 - 1.0 * (clamped_height - 2.0)
            } else if clamped_height <= 3.5 {
                1.0
            } else {
                1.0 + 2.0 * (clamped_height - 3.5)
            }
        } else {
            clamped_height
        };

        self.radius + final_height + 0.1
    }
}
