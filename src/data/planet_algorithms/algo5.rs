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
            let world_x = (v.0 as f64) * radius;
            let world_y = (v.1 as f64) * radius;
            let world_z = (v.2 as f64) * radius;

            let height_base = 0.0;
            let leveled_x = levelize(world_x * 0.007, 1.0, 0.0);
            let leveled_y = levelize(world_y * 0.007, 1.0, 0.0);
            let leveled_z = levelize(world_z * 0.007, 1.0, 0.0);

            let xin =
                leveled_x + noise1.noise_3d(world_x * 0.05, world_y * 0.05, world_z * 0.05) * 0.04;
            let yin =
                leveled_y + noise1.noise_3d(world_y * 0.05, world_z * 0.05, world_x * 0.05) * 0.04;
            let zin =
                leveled_z + noise1.noise_3d(world_z * 0.05, world_x * 0.05, world_y * 0.05) * 0.04;

            let cell_noise = noise2.noise_3d(xin, yin, zin).abs();
            let crack_depth = (0.16 - cell_noise) * 10.0;
            let crack_clamped = if crack_depth > 0.0 {
                if crack_depth > 1.0 {
                    1.0
                } else {
                    crack_depth
                }
            } else {
                0.0
            };
            let crack_intensity = crack_clamped * crack_clamped;

            let fluid_level = (noise1.noise_3d_fbm(
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

            let detail_noise = noise2
                .noise_3d_fbm(xin * 1.5, yin * 1.5, zin * 1.5, 2, 0.5, 2.0)
                .abs();

            let mut terrain_height = height_base - crack_intensity * 1.2 * fluid_clamped;
            if terrain_height >= 0.0 {
                terrain_height += cell_noise * 0.25 + detail_noise * 0.6;
            }

            let mut final_height = terrain_height - 0.1;

            let under_ground = -0.3 - final_height;
            if under_ground > 0.0 {
                let crack_noise =
                    noise2.noise_3d(world_x * 0.16, world_y * 0.16, world_z * 0.16) - 1.0;
                let depth_clamped = if under_ground > 1.0 {
                    1.0
                } else {
                    under_ground
                };
                let depth_power =
                    (3.0 - depth_clamped - depth_clamped) * depth_clamped * depth_clamped;
                final_height = -0.3 - depth_power * 3.7
                    + depth_power * depth_power * depth_power * depth_power * crack_noise * 0.5;
            }

            height_data[i] = ((radius + final_height + 0.2) * 100.0) as u16;
        }

        height_data
    }
}
