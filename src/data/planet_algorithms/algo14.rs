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
        let freq_scale_x: f64 = 0.007;
        let freq_scale_y: f64 = 0.007;
        let freq_scale_z: f64 = 0.007;

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
            let world_x = (v.0 as f64) * radius;
            let world_y = (v.1 as f64) * radius;
            let world_z = (v.2 as f64) * radius;

            let leveled_x = levelize(world_x * 0.007 / 2.0, 1.0, 0.0);
            let leveled_y = levelize(world_y * 0.007 / 2.0, 1.0, 0.0);
            let leveled_z = levelize(world_z * 0.007 / 2.0, 1.0, 0.0);

            let xin =
                leveled_x + noise3.noise_3d(world_x * 0.05, world_y * 0.05, world_z * 0.05) * 0.04;
            let yin =
                leveled_y + noise3.noise_3d(world_y * 0.05, world_z * 0.05, world_x * 0.05) * 0.04;
            let zin =
                leveled_z + noise3.noise_3d(world_z * 0.05, world_x * 0.05, world_y * 0.05) * 0.04;

            let crack_blend = (0.12 - noise4.noise_3d(xin, yin, zin).abs()) * 10.0;
            let crack_clamped = if crack_blend > 0.0 {
                if crack_blend > 1.0 {
                    1.0
                } else {
                    crack_blend
                }
            } else {
                0.0
            };
            let crack_intensity = crack_clamped * crack_clamped;
            let fluid_level = (noise3.noise_3d_fbm(
                world_y * 0.005,
                world_z * 0.005,
                world_x * 0.005,
                4,
                0.5,
                2.0,
            ) + 0.22)
                * 5.0;
            let fluid_clamped = if fluid_level > 0.0 {
                if fluid_level > 1.0 {
                    1.0
                } else {
                    fluid_level
                }
            } else {
                0.0
            };

            // Domain warping
            let warped_x = world_x + (world_y * 0.15).sin() * 3.0;
            let warped_y = world_y + (world_z * 0.15).sin() * 3.0;
            let warped_z = world_z + (warped_x * 0.15).sin() * 3.0;

            let primary_noise = noise1.noise_3d_fbm(
                warped_x * freq_scale_x * 1.0,
                warped_y * freq_scale_y * 1.1,
                warped_z * freq_scale_z * 1.0,
                6,
                0.5,
                1.8,
            );
            let secondary_noise = noise2.noise_3d_fbm(
                warped_x * freq_scale_x * 1.3 + 0.5,
                warped_y * freq_scale_y * 2.8 + 0.2,
                warped_z * freq_scale_z * 1.3 + 0.7,
                3,
                0.5,
                2.0,
            ) * 2.0;
            let reference_noise = noise2.noise_3d_fbm(
                warped_x * freq_scale_x * 0.8,
                warped_y * freq_scale_y * 0.8,
                warped_z * freq_scale_z * 0.8,
                2,
                0.5,
                2.0,
            ) * 2.0;

            let mut f = primary_noise * 2.0
                + 0.92
                + ((secondary_noise * (reference_noise + 0.5).abs() - 0.35) * 1.0).clamp(0.0, 1.0);
            if f < 0.0 {
                f = 0.0;
            }

            let t = levelize2(f, 1.0, 0.0);
            let terrain_base = if t > 0.0 {
                let t2 = levelize2(f, 1.0, 0.0);
                levelize4(t2, 1.0, 0.0)
            } else {
                t
            };

            let height_floor = 0.0;
            let shaped_height = terrain_base;

            let mut combined_height = height_floor - crack_intensity * 1.2 * fluid_clamped;
            if combined_height >= 0.0 {
                combined_height = shaped_height;
            }

            let mut final_height = combined_height - 0.1;

            let under_ground = -0.3 - final_height;
            if under_ground > 0.0 {
                let crater_noise =
                    noise2.noise_3d(warped_x * 0.16, warped_y * 0.16, warped_z * 0.16) - 1.0;
                let depth_clamped = if under_ground > 1.0 {
                    1.0
                } else {
                    under_ground
                };
                let depth_power =
                    (3.0 - depth_clamped - depth_clamped) * depth_clamped * depth_clamped;
                final_height = -0.3 - depth_power * 10.0
                    + depth_power * depth_power * depth_power * depth_power * crater_noise * 0.5;
            }

            height_data[i] = ((radius + final_height + 0.2) * 100.0) as u16;
        }

        height_data
    }
}
