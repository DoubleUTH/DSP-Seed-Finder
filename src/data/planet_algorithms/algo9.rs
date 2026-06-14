use super::super::math::{levelize2, levelize3};
use super::super::planet::Planet;
use crate::data::planet_grid::{get_planet_grid, PlanetGrid};
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm9 - Complex multi-layer noise with modX/modY blending.
pub struct PlanetAlgorithm9 {
    grid: &'static PlanetGrid,
    radius: f64,
    mod_x: f64,
    mod_y: f64,
    noise1: SimplexNoise,
    noise2: SimplexNoise,
}

impl PlanetAlgorithm9 {
    pub fn new(planet: &Planet) -> Self {
        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        Self {
            grid: get_planet_grid(),
            radius: planet.radius as f64,
            mod_x: planet.get_mod_x(),
            mod_y: planet.get_mod_y(),
            noise1: SimplexNoise::with_seed(seed1),
            noise2: SimplexNoise::with_seed(seed2),
        }
    }
}

impl PlanetAlgorithm for PlanetAlgorithm9 {
    fn get_height(&self, index: usize) -> f64 {
        let freq_scale_x: f64 = 0.01;
        let freq_scale_y: f64 = 0.012;
        let freq_scale_z: f64 = 0.01;
        let noise_amplitude: f64 = 3.0;
        let noise_offset: f64 = -0.2;
        let noise2_amplitude: f64 = 0.9;
        let noise2_offset: f64 = 0.5;

        let v = self.grid.get_vertex(index);
        let world_x = (v.0 as f64) * self.radius;
        let world_y = (v.1 as f64) * self.radius;
        let world_z = (v.2 as f64) * self.radius;

        let layer1_noise = self.noise1.noise_3d_fbm(
            world_x * freq_scale_x * 0.75,
            world_y * freq_scale_y * 0.5,
            world_z * freq_scale_z * 0.75,
            6,
            0.5,
            2.0,
        ) * noise_amplitude
            + noise_offset;
        let layer2_noise = self.noise2.noise_3d_fbm(
            world_x * (1.0 / 400.0),
            world_y * (1.0 / 400.0),
            world_z * (1.0 / 400.0),
            3,
            0.5,
            2.0,
        ) * noise_amplitude
            * noise2_amplitude
            + noise2_offset;

        let clamped_layer2 = if layer2_noise > 0.0 {
            layer2_noise * 0.5
        } else {
            layer2_noise
        };
        let combined_noise = layer1_noise + clamped_layer2;
        let f = if combined_noise > 0.0 {
            combined_noise * 0.5
        } else {
            combined_noise * 1.6
        };

        let shaped_height = if f > 0.0 {
            levelize3(f, 0.7, 0.0)
        } else {
            levelize2(f, 0.5, 0.0)
        } + 0.618;

        let stretched_height = if shaped_height > -1.0 {
            shaped_height * 1.5
        } else {
            shaped_height * 4.0
        };

        let layer3_noise = self.noise1.noise_3d_fbm(
            world_x * freq_scale_x * self.mod_x,
            world_y * freq_scale_y * self.mod_x,
            world_z * freq_scale_z * self.mod_x,
            6,
            0.5,
            2.0,
        ) * noise_amplitude
            + noise_offset;
        let layer4_noise = self.noise2.noise_3d_fbm(
            world_x * (1.0 / 400.0),
            world_y * (1.0 / 400.0),
            world_z * (1.0 / 400.0),
            3,
            0.5,
            2.0,
        ) * noise_amplitude
            * noise2_amplitude
            + noise2_offset;
        let clamped_layer4 = if layer4_noise > 0.0 {
            layer4_noise * 0.5
        } else {
            layer4_noise
        };

        let alt_height = ((layer3_noise + clamped_layer4 + 5.0) * 0.13).powf(6.0) * 24.0 - 24.0;

        let blend_factor = if stretched_height >= -self.mod_y {
            0.0
        } else {
            (((stretched_height + self.mod_y).abs() / 5.0).min(1.0)).powf(1.0)
        };

        let blended_height = stretched_height * (1.0 - blend_factor) + alt_height * blend_factor;
        let final_height = if blended_height > 0.0 {
            blended_height * 0.5
        } else {
            blended_height
        };

        self.radius + final_height + 0.2
    }
}
