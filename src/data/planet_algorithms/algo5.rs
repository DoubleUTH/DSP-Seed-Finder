use super::super::math::levelize;
use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm5 - Complex noise with levelized coordinates and cell/crack patterns.
pub struct PlanetAlgorithm5;

impl PlanetAlgorithm for PlanetAlgorithm5 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
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
            let num1 = (v.0 as f64) * radius;
            let num2 = (v.1 as f64) * radius;
            let num3 = (v.2 as f64) * radius;

            let num4 = 0.0;
            let num5 = levelize(num1 * 0.007, 1.0, 0.0);
            let num6 = levelize(num2 * 0.007, 1.0, 0.0);
            let num7 = levelize(num3 * 0.007, 1.0, 0.0);

            let xin = num5 + noise1.noise_3d(num1 * 0.05, num2 * 0.05, num3 * 0.05) * 0.04;
            let yin = num6 + noise1.noise_3d(num2 * 0.05, num3 * 0.05, num1 * 0.05) * 0.04;
            let zin = num7 + noise1.noise_3d(num3 * 0.05, num1 * 0.05, num2 * 0.05) * 0.04;

            let num8 = noise2.noise_3d(xin, yin, zin).abs();
            let num9 = (0.16 - num8) * 10.0;
            let num10 = if num9 > 0.0 {
                if num9 > 1.0 {
                    1.0
                } else {
                    num9
                }
            } else {
                0.0
            };
            let num11 = num10 * num10;

            let num12 =
                (noise1.noise_3d_fbm(num2 * 0.005, num3 * 0.005, num1 * 0.005, 4, 0.5, 2.0) + 0.22)
                    * 5.0;
            let num13 = if num12 > 0.0 {
                if num12 > 1.0 {
                    1.0
                } else {
                    num12
                }
            } else {
                0.0
            };

            let num14 = noise2
                .noise_3d_fbm(xin * 1.5, yin * 1.5, zin * 1.5, 2, 0.5, 2.0)
                .abs();

            let mut num16 = num4 - num11 * 1.2 * num13;
            if num16 >= 0.0 {
                num16 += num8 * 0.25 + num14 * 0.6;
            }

            let mut num17 = num16 - 0.1;

            let num21 = -0.3 - num17;
            if num21 > 0.0 {
                let num22 = noise2.noise_3d(num1 * 0.16, num2 * 0.16, num3 * 0.16) - 1.0;
                let num23 = if num21 > 1.0 { 1.0 } else { num21 };
                let num24 = (3.0 - num23 - num23) * num23 * num23;
                num17 = -0.3 - num24 * 3.7 + num24 * num24 * num24 * num24 * num22 * 0.5;
            }

            height_data[i] = ((radius + num17 + 0.2) * 100.0) as u16;
        }

        height_data
    }
}
