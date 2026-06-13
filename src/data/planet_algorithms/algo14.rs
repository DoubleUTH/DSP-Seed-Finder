use super::super::math::{levelize, levelize2, levelize4};
use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm14 - Lava terrain with domain warping, levelize shaping, and fluid dynamics.
pub struct PlanetAlgorithm14;

impl PlanetAlgorithm for PlanetAlgorithm14 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
        let num1: f64 = 0.007;
        let num2: f64 = 0.007;
        let num3: f64 = 0.007;

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        let seed3 = rand.next_seed();
        let seed4 = rand.next_seed();
        let noise1 = SimplexNoise::with_seed(seed1);
        let noise2 = SimplexNoise::with_seed(seed2);
        let noise3 = SimplexNoise::with_seed(seed3);
        let noise4 = SimplexNoise::with_seed(seed4);

        let data_length = planet_raw_data.data_length();
        let mut height_data = vec![0u16; data_length];
        let radius = planet.radius as f64;

        for i in 0..data_length {
            let v = &planet_raw_data.vertices[i];
            let num4 = (v.0 as f64) * radius;
            let num5 = (v.1 as f64) * radius;
            let num6 = (v.2 as f64) * radius;

            let num7 = levelize(num4 * 0.007 / 2.0, 1.0, 0.0);
            let num8 = levelize(num5 * 0.007 / 2.0, 1.0, 0.0);
            let num9 = levelize(num6 * 0.007 / 2.0, 1.0, 0.0);

            let xin = num7 + noise3.noise_3d(num4 * 0.05, num5 * 0.05, num6 * 0.05) * 0.04;
            let yin = num8 + noise3.noise_3d(num5 * 0.05, num6 * 0.05, num4 * 0.05) * 0.04;
            let zin = num9 + noise3.noise_3d(num6 * 0.05, num4 * 0.05, num5 * 0.05) * 0.04;

            let num10 = (0.12 - noise4.noise_3d(xin, yin, zin).abs()) * 10.0;
            let num11 = if num10 > 0.0 {
                if num10 > 1.0 {
                    1.0
                } else {
                    num10
                }
            } else {
                0.0
            };
            let num12 = num11 * num11;
            let num13 =
                (noise3.noise_3d_fbm(num5 * 0.005, num6 * 0.005, num4 * 0.005, 4, 0.5, 2.0) + 0.22)
                    * 5.0;
            let num14 = if num13 > 0.0 {
                if num13 > 1.0 {
                    1.0
                } else {
                    num13
                }
            } else {
                0.0
            };

            // Domain warping
            let num15 = num4 + (num5 * 0.15).sin() * 3.0;
            let num16 = num5 + (num6 * 0.15).sin() * 3.0;
            let num17 = num6 + (num15 * 0.15).sin() * 3.0;

            let num19 = noise1.noise_3d_fbm(
                num15 * num1 * 1.0,
                num16 * num2 * 1.1,
                num17 * num3 * 1.0,
                6,
                0.5,
                1.8,
            );
            let num20 = noise2.noise_3d_fbm(
                num15 * num1 * 1.3 + 0.5,
                num16 * num2 * 2.8 + 0.2,
                num17 * num3 * 1.3 + 0.7,
                3,
                0.5,
                2.0,
            ) * 2.0;
            let num22 = noise2.noise_3d_fbm(
                num15 * num1 * 0.8,
                num16 * num2 * 0.8,
                num17 * num3 * 0.8,
                2,
                0.5,
                2.0,
            ) * 2.0;

            let mut f =
                num19 * 2.0 + 0.92 + ((num20 * (num22 + 0.5).abs() - 0.35) * 1.0).clamp(0.0, 1.0);
            if f < 0.0 {
                f = 0.0;
            }

            let t = levelize2(f, 1.0, 0.0);
            let num23 = if t > 0.0 {
                let t2 = levelize2(f, 1.0, 0.0);
                levelize4(t2, 1.0, 0.0)
            } else {
                t
            };

            let _a1 = 0.0;
            let num24 = num23;

            let mut num25 = _a1 - num12 * 1.2 * num14;
            if num25 >= 0.0 {
                num25 = num24;
            }

            let mut num26 = num25 - 0.1;

            let num31 = -0.3 - num26;
            if num31 > 0.0 {
                let num32 = noise2.noise_3d(num15 * 0.16, num16 * 0.16, num17 * 0.16) - 1.0;
                let num33 = if num31 > 1.0 { 1.0 } else { num31 };
                let num34 = (3.0 - num33 - num33) * num33 * num33;
                num26 = -0.3 - num34 * 10.0 + num34 * num34 * num34 * num34 * num32 * 0.5;
            }

            height_data[i] = ((radius + num26 + 0.2) * 100.0) as u16;
        }

        height_data
    }
}
