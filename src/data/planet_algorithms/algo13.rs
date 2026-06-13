use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm13 - Noise-based terrain with modX/modY and piecewise height shaping.
pub struct PlanetAlgorithm13;

#[inline]
fn remap(src_min: f64, src_max: f64, tgt_min: f64, tgt_max: f64, x: f64) -> f64 {
    (x - src_min) / (src_max - src_min) * (tgt_max - tgt_min) + tgt_min
}

impl PlanetAlgorithm for PlanetAlgorithm13 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
        let mod_x = planet.get_mod_x();
        let mod_y = planet.get_mod_y();

        let num1 = 0.007 * mod_x;
        let num2 = 0.007 * mod_x;
        let num3 = 0.007 * mod_x;

        let noise = SimplexNoise::with_seed(DspRandom::new(planet.seed).next_seed());

        let data_length = planet_raw_data.data_length();
        let mut height_data = vec![0u16; data_length];
        let radius = planet.radius as f64;

        for i in 0..data_length {
            let v = &planet_raw_data.vertices[i];
            let num4 = (v.0 as f64) * radius;
            let num5 = (v.1 as f64) * radius;
            let num6 = (v.2 as f64) * radius;

            let n = noise.noise_3d_fbm(num4 * num1, num5 * num2, num6 * num3, 6, 0.5, 2.0);
            let mut x_val = remap(
                0.0,
                2.0,
                0.0,
                4.0,
                remap(-1.0, 1.0, 0.0, 1.0, n).powf(mod_y) * (49.0 / 16.0),
            );

            if x_val < 1.0 {
                x_val = x_val.powi(2);
            }

            let num7 = (x_val - 0.2).min(4.0);

            let num8 = if num7 > 2.0 {
                if num7 <= 3.0 {
                    2.0 - 1.0 * (num7 - 2.0)
                } else if num7 <= 3.5 {
                    1.0
                } else {
                    1.0 + 2.0 * (num7 - 3.5)
                }
            } else {
                num7
            };

            height_data[i] = ((radius + num8 + 0.1) * 100.0) as u16;
        }

        height_data
    }
}
