use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm8 - Single noise layer with cosine-based terrain shaping.
/// Uses modX for frequency scaling, modY for height offset.
pub struct PlanetAlgorithm8;

impl PlanetAlgorithm for PlanetAlgorithm8 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
        let mod_x = planet.get_mod_x();
        let mod_y = planet.get_mod_y();

        let freq_scale_x = 0.002 * mod_x;
        let freq_scale_y = 0.002 * mod_x * mod_x * 6.66667;
        let freq_scale_z = 0.002 * mod_x;

        let noise = SimplexNoise::with_seed(DspRandom::new(planet.seed).next_seed());

        let data_length = planet_raw_data.data_length();
        let mut height_data = vec![0u16; data_length];
        let radius = planet.radius as f64;

        for i in 0..data_length {
            let v = &planet_raw_data.vertices[i];
            let world_x = (v.0 as f64) * radius;
            let world_y = (v.1 as f64) * radius;
            let world_z = (v.2 as f64) * radius;

            let noise_val = (noise.noise_3d_fbm(
                world_x * freq_scale_x,
                world_y * freq_scale_y,
                world_z * freq_scale_z,
                6,
                0.45,
                1.8,
            ) + 1.0
                + mod_y * 0.01)
                .clamp(0.0, 2.0);

            let shaped_height = if noise_val < 1.0 {
                let f = (noise_val * std::f64::consts::PI).cos() * 1.1;
                1.0 - ((f.signum() * f.powi(4)).clamp(-1.0, 1.0) + 1.0) * 0.5
            } else {
                let f = ((noise_val - 1.0) * std::f64::consts::PI).cos() * 1.1;
                2.0 - ((f.signum() * f.powi(4)).clamp(-1.0, 1.0) + 1.0) * 0.5
            };

            height_data[i] = ((radius + shaped_height + 0.1) * 100.0) as u16;
        }

        height_data
    }
}
