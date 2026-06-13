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

        let num1 = 0.002 * mod_x;
        let num2 = 0.002 * mod_x * mod_x * 6.66667;
        let num3 = 0.002 * mod_x;

        let noise = SimplexNoise::with_seed(DspRandom::new(planet.seed).next_seed());

        let data_length = planet_raw_data.data_length();
        let mut height_data = vec![0u16; data_length];
        let radius = planet.radius as f64;

        for i in 0..data_length {
            let v = &planet_raw_data.vertices[i];
            let num4 = (v.0 as f64) * radius;
            let num5 = (v.1 as f64) * radius;
            let num6 = (v.2 as f64) * radius;

            let num7 = (noise.noise_3d_fbm(num4 * num1, num5 * num2, num6 * num3, 6, 0.45, 1.8)
                + 1.0
                + mod_y * 0.01)
                .clamp(0.0, 2.0);

            let num9 = if num7 < 1.0 {
                let f = (num7 * std::f64::consts::PI).cos() * 1.1;
                1.0 - ((f.signum() * f.powi(4)).clamp(-1.0, 1.0) + 1.0) * 0.5
            } else {
                let f = ((num7 - 1.0) * std::f64::consts::PI).cos() * 1.1;
                2.0 - ((f.signum() * f.powi(4)).clamp(-1.0, 1.0) + 1.0) * 0.5
            };

            // height = num9
            height_data[i] = ((radius + num9 + 0.1) * 100.0) as u16;
        }

        height_data
    }
}
