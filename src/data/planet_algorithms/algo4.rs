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
        let freq_scale_x: f64 = 0.007;
        let freq_scale_y: f64 = 0.007;
        let freq_scale_z: f64 = 0.007;

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
            let world_x = (v.0 as f64) * radius;
            let world_y = (v.1 as f64) * radius;
            let world_z = (v.2 as f64) * radius;

            let low_freq_noise = noise1.noise_3d_fbm(
                world_x * freq_scale_x,
                world_y * freq_scale_y,
                world_z * freq_scale_z,
                4,
                0.45,
                1.8,
            );
            let high_freq_noise = noise2.noise_3d_fbm(
                world_x * freq_scale_x * 5.0,
                world_y * freq_scale_y * 5.0,
                world_z * freq_scale_z * 5.0,
                4,
                0.5,
                2.0,
            );

            let scaled_low = low_freq_noise * 1.5;
            let scaled_high = high_freq_noise * 0.2;
            let base_elevation = scaled_low * 0.08 + scaled_high * 2.0;

            let mut max_crater = 0.0;
            for j in 0..80 {
                let c = &circles[j];
                let dx = (c.0).0 - world_x;
                let dy = (c.0).1 - world_y;
                let dz = (c.0).2 - world_z;
                let dist_sq = dx * dx + dy * dy + dz * dz;
                if dist_sq <= c.1 {
                    let mut t = dist_sq / c.1 + scaled_high * 1.2;
                    if t < 0.0 {
                        t = 0.0;
                    }
                    let t_sq = t * t;
                    let crater_shape = -15.0 * (t_sq * t) + (131.0 / 6.0) * t_sq
                        - (113.0 / 15.0) * t
                        + 0.7
                        + scaled_high;
                    let crater_shape = if crater_shape < 0.0 {
                        0.0
                    } else {
                        crater_shape
                    };
                    let crater_val = crater_shape * crater_shape * heights[j];
                    if crater_val > max_crater {
                        max_crater = crater_val;
                    }
                }
            }

            let final_height = max_crater + base_elevation + 0.2;
            height_data[i] = ((radius + final_height + 0.1) * 100.0) as u16;
        }

        height_data
    }
}
