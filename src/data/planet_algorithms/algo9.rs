use super::super::math::{levelize2, levelize3};
use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm9 - Complex multi-layer noise with modX/modY blending.
#[derive(Default)]
pub struct PlanetAlgorithm9 {
    radius: f32,
    mod_x: f64,
    mod_y: f64,
    noise1: Option<SimplexNoise>,
    noise2: Option<SimplexNoise>,
}

impl PlanetAlgorithm for PlanetAlgorithm9 {
    fn prepare_data(&mut self, planet: &Planet) {
        self.radius = planet.radius;
        self.mod_x = planet.get_mod_x();
        self.mod_y = planet.get_mod_y();

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        self.noise1 = Some(SimplexNoise::with_seed(seed1));
        self.noise2 = Some(SimplexNoise::with_seed(seed2));
    }

    fn get_height(&self, index: usize, planet_raw_data: &PlanetRawData) -> f32 {
        let freq_scale_x: f64 = 0.01;
        let freq_scale_y: f64 = 0.012;
        let freq_scale_z: f64 = 0.01;
        let noise_amplitude: f64 = 3.0;
        let noise_offset: f64 = -0.2;
        let noise2_amplitude: f64 = 0.9;
        let noise2_offset: f64 = 0.5;

        let v = &planet_raw_data.vertices[index];
        let world_x = (v.0 as f64) * self.radius as f64;
        let world_y = (v.1 as f64) * self.radius as f64;
        let world_z = (v.2 as f64) * self.radius as f64;

        let noise1 = self.noise1.as_ref().unwrap();
        let noise2 = self.noise2.as_ref().unwrap();

        let layer1_noise = noise1.noise_3d_fbm(
            world_x * freq_scale_x * 0.75,
            world_y * freq_scale_y * 0.5,
            world_z * freq_scale_z * 0.75,
            6,
            0.5,
            2.0,
        ) * noise_amplitude
            + noise_offset;
        let layer2_noise = noise2.noise_3d_fbm(
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

        let layer3_noise = noise1.noise_3d_fbm(
            world_x * freq_scale_x * self.mod_x,
            world_y * freq_scale_y * self.mod_x,
            world_z * freq_scale_z * self.mod_x,
            6,
            0.5,
            2.0,
        ) * noise_amplitude
            + noise_offset;
        let layer4_noise = noise2.noise_3d_fbm(
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

        (self.radius as f64 + final_height + 0.2) as f32
    }
}
