use super::super::math::{levelize2, levelize3};
use super::super::planet::Planet;
use super::super::planet_raw_data::get_vertex;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm7 - Similar to algo1 but without +0.2 offset in height and different constants.
pub struct PlanetAlgorithm7 {
    radius: f32,
    noise1: SimplexNoise,
    noise2: SimplexNoise,
}

impl PlanetAlgorithm7 {
    pub fn new(planet: &Planet) -> Self {
        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        Self {
            radius: planet.radius,
            noise1: SimplexNoise::with_seed(seed1),
            noise2: SimplexNoise::with_seed(seed2),
        }
    }
}

impl PlanetAlgorithm for PlanetAlgorithm7 {
    fn get_height(&self, index: usize) -> f32 {
        let freq_scale_x: f64 = 0.008;
        let freq_scale_y: f64 = 0.01;
        let freq_scale_z: f64 = 0.01;
        let noise_amplitude: f64 = 3.0;
        let noise_offset: f64 = -2.4;
        let noise2_amplitude: f64 = 0.9;
        let noise2_offset: f64 = 0.5;

        let v = get_vertex(index);
        let world_x = (v.0 as f64) * self.radius as f64;
        let world_y = (v.1 as f64) * self.radius as f64;
        let world_z = (v.2 as f64) * self.radius as f64;

        let layer1_noise = self.noise1.noise_3d_fbm(
            world_x * freq_scale_x,
            world_y * freq_scale_y,
            world_z * freq_scale_z,
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
        };

        (self.radius as f64 + shaped_height) as f32
    }
}
