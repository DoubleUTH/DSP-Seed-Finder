use super::super::math::{levelize, levelize4};
use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::random_table::RandomTable;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm10 - FBM noise with 10 elliptical crater features.
pub struct PlanetAlgorithm10;

#[inline]
fn remap(src_min: f64, src_max: f64, tgt_min: f64, tgt_max: f64, x: f64) -> f64 {
    (x - src_min) / (src_max - src_min) * (tgt_max - tgt_min) + tgt_min
}

impl PlanetAlgorithm for PlanetAlgorithm10 {
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
        // Pre-compute 10 ellipses using RandomTable.SphericNormal with seed5
        let mut seed5 = rand.next_seed();

        let mut ellipses = Vec::with_capacity(10);
        let mut eccentricities = Vec::with_capacity(10);
        let mut heights = Vec::with_capacity(10);

        for _ in 0..10 {
            let mut v = RandomTable::spheric_normal(&mut seed5, 1.0);
            v.normalize();
            v *= planet.radius as f64;
            let w = (rand.next_f64() * 10.0 + 40.0) as f32;
            ellipses.push((v, (w * w) as f64));

            let ecc = if rand.next_f64() <= 0.5 {
                remap(0.0, 1.0, 0.2, 1.0 / 3.0, rand.next_f64())
            } else {
                remap(0.0, 1.0, 3.0, 5.0, rand.next_f64())
            };
            eccentricities.push(ecc);

            let h = remap(0.0, 1.0, 1.0, 2.0, rand.next_f64());
            heights.push(h);
        }

        let data_length = planet_raw_data.data_length();
        let mut height_data = vec![0u16; data_length];
        let radius = planet.radius as f64;

        for i in 0..data_length {
            let v = &planet_raw_data.vertices[i];
            let num4 = (v.0 as f64) * radius;
            let num5 = (v.1 as f64) * radius;
            let num6 = (v.2 as f64) * radius;

            let num7 = levelize(num4 * 0.007, 1.0, 0.0);
            let num8 = levelize(num5 * 0.007, 1.0, 0.0);
            let num9 = levelize(num6 * 0.007, 1.0, 0.0);

            let xin = num7 + noise3.noise_3d(num4 * 0.05, num5 * 0.05, num6 * 0.05) * 0.04;
            let yin = num8 + noise3.noise_3d(num5 * 0.05, num6 * 0.05, num4 * 0.05) * 0.04;
            let zin = num9 + noise3.noise_3d(num6 * 0.05, num4 * 0.05, num5 * 0.05) * 0.04;

            let num10 = noise4.noise_3d(xin, yin, zin).abs();
            let num11 = (0.16 - num10) * 10.0;
            let num12 = if num11 > 0.0 {
                if num11 > 1.0 {
                    1.0
                } else {
                    num11
                }
            } else {
                0.0
            };
            let num13 = num12 * num12;

            let num14 =
                (noise3.noise_3d_fbm(num5 * 0.005, num6 * 0.005, num4 * 0.005, 4, 0.5, 2.0) + 0.22)
                    * 5.0;
            let num15 = if num14 > 0.0 {
                if num14 > 1.0 {
                    1.0
                } else {
                    num14
                }
            } else {
                0.0
            };

            let num16 = noise4
                .noise_3d_fbm(xin * 1.5, yin * 1.5, zin * 1.5, 2, 0.5, 2.0)
                .abs();
            let x_val = noise2.noise_3d_fbm(
                num4 * num1 * 5.0,
                num5 * num2 * 5.0,
                num6 * num3 * 5.0,
                4,
                0.5,
                2.0,
            );
            let num17 = x_val * 0.2;

            // Ellipse contributions
            let mut a1 = 0.0;
            for j in 0..10 {
                let e = &ellipses[j];
                let ecc = eccentricities[j];
                let num18 = (e.0).0 - num4;
                let num19 = (e.0).1 - num5;
                let num20 = (e.0).2 - num6;
                let num21 = ecc * num18 * num18 + num19 * num19 + num20 * num20;
                let num22 = remap(-1.0, 1.0, 0.2, 5.0, x_val) * num21;
                if num22 < e.1 {
                    let sqrt_val = (num22 / e.1).sqrt();
                    let num23 = 1.0 - (1.0 - sqrt_val);
                    let mut num24 = 1.0 - num23 * num23 * num23 * num23 + num17 * 2.0;
                    if num24 < 0.0 {
                        num24 = 0.0;
                    }
                    let candidate = heights[j] * num24;
                    if candidate > a1 {
                        a1 = candidate;
                    }
                }
            }

            // Domain warping
            let num25 = num4 + (num5 * 0.15).sin() * 2.0;
            let num26 = num5 + (num6 * 0.15).sin() * 2.0;
            let num27 = num6 + (num25 * 0.15).sin() * 2.0;

            let num28 = num25 * num1;
            let num29 = num26 * num2;
            let num30 = num27 * num3;

            let f_val = ((noise1.noise_3d_fbm(num28 * 0.6, num29 * 0.6, num30 * 0.6, 4, 0.5, 1.8)
                + 1.0)
                * 0.5)
                .powf(1.3);

            let num31 = remap(
                -1.0,
                1.0,
                -0.1,
                0.15,
                noise2.noise_3d_fbm(num28 * 6.0, num29 * 6.0, num30 * 6.0, 5, 0.5, 2.0),
            );

            let num32 =
                noise2.noise_3d_fbm(num28 * 5.0 * 3.0, num29 * 5.0, num30 * 5.0, 1, 0.5, 2.0);
            let num33 = noise2.noise_3d_fbm(
                num28 * 5.0 * 3.0 + num32 * 0.3,
                num29 * 5.0 + num32 * 0.3,
                num30 * 5.0 + num32 * 0.3,
                5,
                0.5,
                2.0,
            ) * 0.1;

            let mut num34 = (levelize(levelize4(f_val, 1.0, 0.0), 1.0, 0.0)).min(1.0);
            if num34 <= 0.8 {
                if num34 > 0.4 {
                    num34 += num33;
                } else {
                    num34 += num31;
                }
            }

            let num35 = (num34 * 2.5 - num34 * a1).max(num31 * 2.0);
            let num36 = (2.0 - num35) / 2.0;
            let mut num37 = num35 - num13 * 1.2 * num15 * num36;
            if num37 >= 0.0 {
                num37 += (num10 * 0.25 + num16 * 0.6) * num36;
            }
            let a2 = num37 - 0.1;

            height_data[i] = ((radius + a2 + 0.1) * 100.0) as u16;
        }

        height_data
    }
}
