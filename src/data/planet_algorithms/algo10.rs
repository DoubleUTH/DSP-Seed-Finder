use super::super::math::{levelize, levelize4};
use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::random_table::RandomTable;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm10 - FBM noise with 10 elliptical crater features.
#[derive(Default)]
pub struct PlanetAlgorithm10 {
    radius: f32,
    noise1: Option<SimplexNoise>,
    noise2: Option<SimplexNoise>,
    noise3: Option<SimplexNoise>,
    noise4: Option<SimplexNoise>,
    ellipses: Option<Vec<([f64; 3], f64)>>,
    eccentricities: Option<Vec<f64>>,
    heights: Option<Vec<f64>>,
}

#[inline]
fn remap(src_min: f64, src_max: f64, tgt_min: f64, tgt_max: f64, x: f64) -> f64 {
    (x - src_min) / (src_max - src_min) * (tgt_max - tgt_min) + tgt_min
}

impl PlanetAlgorithm for PlanetAlgorithm10 {
    fn prepare_data(&mut self, planet: &Planet) {
        self.radius = planet.radius;

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        let seed3 = rand.next_seed();
        let seed4 = rand.next_seed();
        self.noise1 = Some(SimplexNoise::with_seed(seed1));
        self.noise2 = Some(SimplexNoise::with_seed(seed2));
        self.noise3 = Some(SimplexNoise::with_seed(seed3));
        self.noise4 = Some(SimplexNoise::with_seed(seed4));

        let mut seed5 = rand.next_seed();
        let mut els = Vec::with_capacity(10);
        let mut eccs = Vec::with_capacity(10);
        let mut hs = Vec::with_capacity(10);

        for _ in 0..10 {
            let mut v = RandomTable::spheric_normal(&mut seed5, 1.0);
            v.normalize();
            v *= planet.radius as f64;
            let w = (rand.next_f64() * 10.0 + 40.0) as f32;
            els.push(([v.0, v.1, v.2], (w * w) as f64));

            let ecc = if rand.next_f64() <= 0.5 {
                remap(0.0, 1.0, 0.2, 1.0 / 3.0, rand.next_f64())
            } else {
                remap(0.0, 1.0, 3.0, 5.0, rand.next_f64())
            };
            eccs.push(ecc);

            let h = remap(0.0, 1.0, 1.0, 2.0, rand.next_f64());
            hs.push(h);
        }

        self.ellipses = Some(els);
        self.eccentricities = Some(eccs);
        self.heights = Some(hs);
    }

    fn get_height(&self, index: usize, planet_raw_data: &PlanetRawData) -> f32 {
        let freq_scale_x: f64 = 0.007;
        let freq_scale_y: f64 = 0.007;
        let freq_scale_z: f64 = 0.007;

        let v = &planet_raw_data.vertices[index];
        let world_x = (v.0 as f64) * self.radius as f64;
        let world_y = (v.1 as f64) * self.radius as f64;
        let world_z = (v.2 as f64) * self.radius as f64;

        let noise1 = self.noise1.as_ref().unwrap();
        let noise2 = self.noise2.as_ref().unwrap();
        let noise3 = self.noise3.as_ref().unwrap();
        let noise4 = self.noise4.as_ref().unwrap();

        let leveled_x = levelize(world_x * 0.007, 1.0, 0.0);
        let leveled_y = levelize(world_y * 0.007, 1.0, 0.0);
        let leveled_z = levelize(world_z * 0.007, 1.0, 0.0);

        let xin =
            leveled_x + noise3.noise_3d(world_x * 0.05, world_y * 0.05, world_z * 0.05) * 0.04;
        let yin =
            leveled_y + noise3.noise_3d(world_y * 0.05, world_z * 0.05, world_x * 0.05) * 0.04;
        let zin =
            leveled_z + noise3.noise_3d(world_z * 0.05, world_x * 0.05, world_y * 0.05) * 0.04;

        let cell_noise = noise4.noise_3d(xin, yin, zin).abs();
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

        let detail_noise = noise4
            .noise_3d_fbm(xin * 1.5, yin * 1.5, zin * 1.5, 2, 0.5, 2.0)
            .abs();
        let x_noise_val = noise2.noise_3d_fbm(
            world_x * freq_scale_x * 5.0,
            world_y * freq_scale_y * 5.0,
            world_z * freq_scale_z * 5.0,
            4,
            0.5,
            2.0,
        );
        let high_freq_amp = x_noise_val * 0.2;

        // Ellipse contributions
        let mut max_crater = 0.0;
        let ellipses = self.ellipses.as_ref().unwrap();
        let eccentricities = self.eccentricities.as_ref().unwrap();
        let heights = self.heights.as_ref().unwrap();
        for j in 0..10 {
            let e = &ellipses[j];
            let ecc = eccentricities[j];
            let dx = e.0[0] - world_x;
            let dy = e.0[1] - world_y;
            let dz = e.0[2] - world_z;
            let dist_ecc = ecc * dx * dx + dy * dy + dz * dz;
            let dist_scaled = remap(-1.0, 1.0, 0.2, 5.0, x_noise_val) * dist_ecc;
            if dist_scaled < e.1 {
                let sqrt_val = (dist_scaled / e.1).sqrt();
                let crater_t = 1.0 - (1.0 - sqrt_val);
                let mut crater_shape =
                    1.0 - crater_t * crater_t * crater_t * crater_t + high_freq_amp * 2.0;
                if crater_shape < 0.0 {
                    crater_shape = 0.0;
                }
                let candidate = heights[j] * crater_shape;
                if candidate > max_crater {
                    max_crater = candidate;
                }
            }
        }

        // Domain warping
        let warped_x = world_x + (world_y * 0.15).sin() * 2.0;
        let warped_y = world_y + (world_z * 0.15).sin() * 2.0;
        let warped_z = world_z + (warped_x * 0.15).sin() * 2.0;

        let warp_x_scaled = warped_x * freq_scale_x;
        let warp_y_scaled = warped_y * freq_scale_y;
        let warp_z_scaled = warped_z * freq_scale_z;

        let f_val = ((noise1.noise_3d_fbm(
            warp_x_scaled * 0.6,
            warp_y_scaled * 0.6,
            warp_z_scaled * 0.6,
            4,
            0.5,
            1.8,
        ) + 1.0)
            * 0.5)
            .powf(1.3);

        let remap_noise = remap(
            -1.0,
            1.0,
            -0.1,
            0.15,
            noise2.noise_3d_fbm(
                warp_x_scaled * 6.0,
                warp_y_scaled * 6.0,
                warp_z_scaled * 6.0,
                5,
                0.5,
                2.0,
            ),
        );

        let turb_base = noise2.noise_3d_fbm(
            warp_x_scaled * 5.0 * 3.0,
            warp_y_scaled * 5.0,
            warp_z_scaled * 5.0,
            1,
            0.5,
            2.0,
        );
        let turb_detail = noise2.noise_3d_fbm(
            warp_x_scaled * 5.0 * 3.0 + turb_base * 0.3,
            warp_y_scaled * 5.0 + turb_base * 0.3,
            warp_z_scaled * 5.0 + turb_base * 0.3,
            5,
            0.5,
            2.0,
        ) * 0.1;

        let mut shaped = (levelize(levelize4(f_val, 1.0, 0.0), 1.0, 0.0)).min(1.0);
        if shaped <= 0.8 {
            if shaped > 0.4 {
                shaped += turb_detail;
            } else {
                shaped += remap_noise;
            }
        }

        let crater_blend = (shaped * 2.5 - shaped * max_crater).max(remap_noise * 2.0);
        let crack_scale = (2.0 - crater_blend) / 2.0;
        let mut terrain_height = crater_blend - crack_intensity * 1.2 * fluid_clamped * crack_scale;
        if terrain_height >= 0.0 {
            terrain_height += (cell_noise * 0.25 + detail_noise * 0.6) * crack_scale;
        }
        let final_height = terrain_height - 0.1;

        (self.radius as f64 + final_height + 0.1) as f32
    }
}
