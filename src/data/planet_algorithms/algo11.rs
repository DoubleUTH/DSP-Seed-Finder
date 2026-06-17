use super::super::math::{levelize2, levelize3};
use super::super::planet::Planet;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;
use crate::data::planet_grid::{get_planet_grid, PlanetGrid};

/// PlanetAlgorithm11 - Complex noise with Remap, Levelize2/Levelize3 and modX/modY.
pub struct PlanetAlgorithm11 {
    grid: &'static PlanetGrid,
    radius: f64,
    mod_y: f64,
    noise1: SimplexNoise,
    noise2: SimplexNoise,
    noise3: SimplexNoise,
    mod_freq_x: f64,
    mod_freq_y: f64,
    mod_freq_z: f64,
}

#[inline]
fn remap(src_min: f64, src_max: f64, tgt_min: f64, tgt_max: f64, x: f64) -> f64 {
    (x - src_min) / (src_max - src_min) * (tgt_max - tgt_min) + tgt_min
}

impl PlanetAlgorithm11 {
    pub fn new(planet: &Planet) -> Self {
        let mod_x = planet.get_mod_x();
        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        let seed3 = rand.next_seed();
        Self {
            grid: get_planet_grid(),
            radius: planet.radius as f64,
            mod_y: planet.get_mod_y(),
            noise1: SimplexNoise::with_seed(seed1),
            noise2: SimplexNoise::with_seed(seed2),
            noise3: SimplexNoise::with_seed(seed3),
            mod_freq_x: 0.002 * mod_x,
            mod_freq_y: 0.002 * mod_x * 4.0,
            mod_freq_z: 0.002 * mod_x,
        }
    }
}

impl PlanetAlgorithm for PlanetAlgorithm11 {
    fn get_height(&self, index: usize) -> f64 {
        let freq_scale_x: f64 = 0.007;
        let freq_scale_y: f64 = 0.007;
        let freq_scale_z: f64 = 0.007;

        let v = self.grid.get_vertex(index);
        let world_x = (v.0 as f64) * self.radius;
        let world_y = (v.1 as f64) * self.radius;
        let world_z = (v.2 as f64) * self.radius;

        let detail_noise = self.noise2.noise_3d_fbm(
            world_x * freq_scale_x * 4.0,
            world_y * freq_scale_y * 8.0,
            world_z * freq_scale_z * 4.0,
            3,
            0.5,
            2.0,
        );
        let primary_freq_scale = 0.6;

        let inner = self.noise1.noise_3d_fbm(
            world_x * freq_scale_x * primary_freq_scale,
            world_y * freq_scale_x * 1.5 * 2.5,
            world_z * freq_scale_x * primary_freq_scale,
            6,
            0.45,
            1.8,
        ) * 0.95
            + detail_noise * 0.05;

        let primary_shaped = levelize2(
            (remap(-1.0, 1.0, 0.0, 1.0, inner)).powf(self.mod_y) + 1.0,
            1.0,
            0.0,
        );

        let inner2 = self.noise3.noise_3d_fbm(
            world_x * self.mod_freq_x,
            world_y * self.mod_freq_y,
            world_z * self.mod_freq_z,
            5,
            0.55,
            2.0,
        );
        let secondary_shaped =
            levelize3((remap(-1.0, 1.0, 0.0, 1.0, inner2)).powf(0.65), 1.0, 0.0) * primary_shaped;

        let final_height = ((secondary_shaped - 0.4) * 0.9).max(-0.3);

        self.radius + final_height
    }
}
