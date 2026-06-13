use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::random_table::RandomTable;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm4 - FBM noise with 80 circular crater features.
/// Uses RandomTable.SphericNormal for crater placement.
pub struct PlanetAlgorithm4;

impl PlanetAlgorithm for PlanetAlgorithm4 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
        let num1: f64 = 0.007;
        let num2: f64 = 0.007;
        let num3: f64 = 0.007;

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        // Pre-compute 80 circles using RandomTable.SphericNormal with seed3
        let mut seed3 = rand.next_seed();
        let noise1 = SimplexNoise::with_seed(seed1);
        let noise2 = SimplexNoise::with_seed(seed2);

        let mut circles = Vec::with_capacity(80);
        let mut heights = Vec::with_capacity(80);
        let radius = planet.radius as f64;

        for _ in 0..80 {
            let mut v = RandomTable::spheric_normal(&mut seed3, 1.0);
            let w = (v.magnitude() * 8.0 + 8.0) as f32;
            v.normalize();
            v *= planet.radius as f64;
            circles.push((v, (w * w) as f64));

            let h = rand.next_f64() * 0.4 + 0.2;
            heights.push(h);
        }

        let data_length = planet_raw_data.data_length();
        let mut height_data = vec![0u16; data_length];

        for i in 0..data_length {
            let v = &planet_raw_data.vertices[i];
            let num4 = (v.0 as f64) * radius;
            let num5 = (v.1 as f64) * radius;
            let num6 = (v.2 as f64) * radius;

            let num7 = noise1.noise_3d_fbm(num4 * num1, num5 * num2, num6 * num3, 4, 0.45, 1.8);
            let num8 = noise2.noise_3d_fbm(
                num4 * num1 * 5.0,
                num5 * num2 * 5.0,
                num6 * num3 * 5.0,
                4,
                0.5,
                2.0,
            );

            let num9 = num7 * 1.5;
            let num10 = num8 * 0.2;
            let num11 = num9 * 0.08 + num10 * 2.0;

            let mut num12 = 0.0;
            for j in 0..80 {
                let c = &circles[j];
                let num13 = (c.0).0 - num4;
                let num14 = (c.0).1 - num5;
                let num15 = (c.0).2 - num6;
                let num16 = num13 * num13 + num14 * num14 + num15 * num15;
                if num16 <= c.1 {
                    let mut num17 = num16 / c.1 + num10 * 1.2;
                    if num17 < 0.0 {
                        num17 = 0.0;
                    }
                    let num18 = num17 * num17;
                    let num19 = -15.0 * (num18 * num17) + (131.0 / 6.0) * num18
                        - (113.0 / 15.0) * num17
                        + 0.7
                        + num10;
                    let num19 = if num19 < 0.0 { 0.0 } else { num19 };
                    let num20 = num19 * num19 * heights[j];
                    if num20 > num12 {
                        num12 = num20;
                    }
                }
            }

            let num21 = num12 + num11 + 0.2;
            height_data[i] = ((radius + num21 + 0.1) * 100.0) as u16;
        }

        height_data
    }
}
