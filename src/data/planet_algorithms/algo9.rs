use super::super::math::{levelize2, levelize3};
use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm9 - Complex multi-layer noise with modX/modY blending.
pub struct PlanetAlgorithm9;

impl PlanetAlgorithm for PlanetAlgorithm9 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
        let mod_x = planet.get_mod_x();
        let mod_y = planet.get_mod_y();

        let num1: f64 = 0.01;
        let num2: f64 = 0.012;
        let num3: f64 = 0.01;
        let num4: f64 = 3.0;
        let num5: f64 = -0.2;
        let num6: f64 = 0.9;
        let num7: f64 = 0.5;

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        let noise1 = SimplexNoise::with_seed(seed1);
        let noise2 = SimplexNoise::with_seed(seed2);

        let data_length = planet_raw_data.data_length();
        let mut height_data = vec![0u16; data_length];
        let radius = planet.radius as f64;

        for i in 0..data_length {
            let v = &planet_raw_data.vertices[i];
            let num10 = (v.0 as f64) * radius;
            let num11 = (v.1 as f64) * radius;
            let num12 = (v.2 as f64) * radius;

            let num13 = noise1.noise_3d_fbm(
                num10 * num1 * 0.75,
                num11 * num2 * 0.5,
                num12 * num3 * 0.75,
                6,
                0.5,
                2.0,
            ) * num4
                + num5;
            let num14 = noise2.noise_3d_fbm(
                num10 * (1.0 / 400.0),
                num11 * (1.0 / 400.0),
                num12 * (1.0 / 400.0),
                3,
                0.5,
                2.0,
            ) * num4
                * num6
                + num7;

            let num15 = if num14 > 0.0 { num14 * 0.5 } else { num14 };
            let num16 = num13 + num15;
            let f = if num16 > 0.0 {
                num16 * 0.5
            } else {
                num16 * 1.6
            };

            let num17 = if f > 0.0 {
                levelize3(f, 0.7, 0.0)
            } else {
                levelize2(f, 0.5, 0.0)
            } + 0.618;

            let num18 = if num17 > -1.0 {
                num17 * 1.5
            } else {
                num17 * 4.0
            };

            let num21 = noise1.noise_3d_fbm(
                num10 * num1 * mod_x,
                num11 * num2 * mod_x,
                num12 * num3 * mod_x,
                6,
                0.5,
                2.0,
            ) * num4
                + num5;
            let num22 = noise2.noise_3d_fbm(
                num10 * (1.0 / 400.0),
                num11 * (1.0 / 400.0),
                num12 * (1.0 / 400.0),
                3,
                0.5,
                2.0,
            ) * num4
                * num6
                + num7;
            let num23 = if num22 > 0.0 { num22 * 0.5 } else { num22 };

            let num24 = ((num21 + num23 + 5.0) * 0.13).powf(6.0) * 24.0 - 24.0;

            let num25 = if num18 >= -mod_y {
                0.0
            } else {
                (((num18 + mod_y).abs() / 5.0).min(1.0)).powf(1.0)
            };

            let num26 = num18 * (1.0 - num25) + num24 * num25;
            let num27 = if num26 > 0.0 { num26 * 0.5 } else { num26 };

            height_data[i] = ((radius + num27 + 0.2) * 100.0) as u16;
        }

        height_data
    }
}
