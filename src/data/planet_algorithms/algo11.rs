use super::super::math::{levelize2, levelize3};
use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm11 - Complex noise with Remap, Levelize2/Levelize3 and modX/modY.
pub struct PlanetAlgorithm11;

#[inline]
fn remap(src_min: f64, src_max: f64, tgt_min: f64, tgt_max: f64, x: f64) -> f64 {
    (x - src_min) / (src_max - src_min) * (tgt_max - tgt_min) + tgt_min
}

impl PlanetAlgorithm for PlanetAlgorithm11 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
        let mod_x = planet.get_mod_x();
        let mod_y = planet.get_mod_y();

        let num1: f64 = 0.007;
        let num2: f64 = 0.007;
        let num3: f64 = 0.007;
        let num4 = 0.002 * mod_x;
        let num5 = 0.002 * mod_x * 4.0;
        let num6 = 0.002 * mod_x;

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        let seed3 = rand.next_seed();
        let noise1 = SimplexNoise::with_seed(seed1);
        let noise2 = SimplexNoise::with_seed(seed2);
        let noise3 = SimplexNoise::with_seed(seed3);

        let data_length = planet_raw_data.data_length();
        let mut height_data = vec![0u16; data_length];
        let radius = planet.radius as f64;

        for i in 0..data_length {
            let v = &planet_raw_data.vertices[i];
            let num7 = (v.0 as f64) * radius;
            let num8 = (v.1 as f64) * radius;
            let num9 = (v.2 as f64) * radius;

            let num10 = noise2.noise_3d_fbm(
                num7 * num1 * 4.0,
                num8 * num2 * 8.0,
                num9 * num3 * 4.0,
                3,
                0.5,
                2.0,
            );
            let num11_const = 0.6;

            let inner = noise1.noise_3d_fbm(
                num7 * num1 * num11_const,
                num8 * num1 * 1.5 * 2.5,
                num9 * num1 * num11_const,
                6,
                0.45,
                1.8,
            ) * 0.95
                + num10 * 0.05;

            let num12 = levelize2(
                (remap(-1.0, 1.0, 0.0, 1.0, inner)).powf(mod_y) + 1.0,
                1.0,
                0.0,
            );

            let inner2 = noise3.noise_3d_fbm(num7 * num4, num8 * num5, num9 * num6, 5, 0.55, 2.0);
            let num13 =
                levelize3((remap(-1.0, 1.0, 0.0, 1.0, inner2)).powf(0.65), 1.0, 0.0) * num12;

            let num14 = ((num13 - 0.4) * 0.9).max(-0.3);

            height_data[i] = ((radius + num14) * 100.0) as u16;
        }

        height_data
    }
}
