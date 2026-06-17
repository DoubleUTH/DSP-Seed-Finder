use super::super::math::levelize;
use super::super::planet::Planet;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;
use crate::data::planet_grid::{get_planet_grid, PlanetGrid};

/// PlanetAlgorithm5 - Complex noise with levelized coordinates and cell/crack patterns.
pub struct PlanetAlgorithm5 {
    grid: &'static PlanetGrid,
    radius: f64,
    noise1: SimplexNoise,
    noise2: SimplexNoise,
}

impl PlanetAlgorithm5 {
    pub fn new(planet: &Planet) -> Self {
        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        Self {
            grid: get_planet_grid(),
            radius: planet.radius as f64,
            noise1: SimplexNoise::with_seed(seed1),
            noise2: SimplexNoise::with_seed(seed2),
        }
    }
}

impl PlanetAlgorithm for PlanetAlgorithm5 {
    fn get_height(&self, index: usize) -> f64 {
        let v = self.grid.get_vertex(index);
        let world_x = (v.0 as f64) * self.radius;
        let world_y = (v.1 as f64) * self.radius;
        let world_z = (v.2 as f64) * self.radius;

        let height_base = 0.0;
        let leveled_x = levelize(world_x * 0.007, 1.0, 0.0);
        let leveled_y = levelize(world_y * 0.007, 1.0, 0.0);
        let leveled_z = levelize(world_z * 0.007, 1.0, 0.0);

        let xin = leveled_x
            + self
                .noise1
                .noise_3d(world_x * 0.05, world_y * 0.05, world_z * 0.05)
                * 0.04;
        let yin = leveled_y
            + self
                .noise1
                .noise_3d(world_y * 0.05, world_z * 0.05, world_x * 0.05)
                * 0.04;
        let zin = leveled_z
            + self
                .noise1
                .noise_3d(world_z * 0.05, world_x * 0.05, world_y * 0.05)
                * 0.04;

        let cell_noise = self.noise2.noise_3d(xin, yin, zin).abs();
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

        let fluid_level = (self.noise1.noise_3d_fbm(
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

        let detail_noise = self
            .noise2
            .noise_3d_fbm(xin * 1.5, yin * 1.5, zin * 1.5, 2, 0.5, 2.0)
            .abs();

        let mut terrain_height = height_base - crack_intensity * 1.2 * fluid_clamped;
        if terrain_height >= 0.0 {
            terrain_height += cell_noise * 0.25 + detail_noise * 0.6;
        }

        let mut final_height = terrain_height - 0.1;

        let under_ground = -0.3 - final_height;
        if under_ground > 0.0 {
            let crack_noise = self
                .noise2
                .noise_3d(world_x * 0.16, world_y * 0.16, world_z * 0.16)
                - 1.0;
            let depth_clamped = if under_ground > 1.0 {
                1.0
            } else {
                under_ground
            };
            let depth_power = (3.0 - depth_clamped - depth_clamped) * depth_clamped * depth_clamped;
            final_height = -0.3 - depth_power * 3.7
                + depth_power * depth_power * depth_power * depth_power * crack_noise * 0.5;
        }

        self.radius + final_height + 0.2
    }
}
