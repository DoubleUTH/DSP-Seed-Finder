use super::super::math::{levelize2, levelize3};
use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm7 - Similar to algo1 but without +0.2 offset in height and different constants.
pub struct PlanetAlgorithm7;

impl PlanetAlgorithm for PlanetAlgorithm7 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
        let num1: f64 = 0.008;
        let num2: f64 = 0.01;
        let num3: f64 = 0.01;
        let num4: f64 = 3.0;
        let num5: f64 = -2.4;
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

            let num13 = noise1.noise_3d_fbm(num10 * num1, num11 * num2, num12 * num3, 6, 0.5, 2.0)
                * num4
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
            };

            // height = num17, no +0.2 offset
            height_data[i] = ((radius + num17) * 100.0) as u16;
        }

        height_data
    }
}
