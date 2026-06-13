use super::super::math::{levelize2, levelize4};
use super::super::planet::Planet;
use super::super::planet_raw_data::get_vertex;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

#[inline]
fn lerp_nc(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// PlanetAlgorithm3 - Complex FBM noise with domain warping and multi-level shaping.
pub struct PlanetAlgorithm3 {
    radius: f32,
    mod_x: f64,
    noise1: SimplexNoise,
    noise2: SimplexNoise,
}

impl PlanetAlgorithm3 {
    pub fn new(planet: &Planet) -> Self {
        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        Self {
            radius: planet.radius,
            mod_x: planet.get_mod_x(),
            noise1: SimplexNoise::with_seed(seed1),
            noise2: SimplexNoise::with_seed(seed2),
        }
    }
}

impl PlanetAlgorithm for PlanetAlgorithm3 {
    fn get_height(&self, index: usize) -> f32 {
        let freq_scale_x: f64 = 0.007;
        let freq_scale_y: f64 = 0.007;
        let freq_scale_z: f64 = 0.007;

        let v = get_vertex(index);
        let world_x = (v.0 as f64) * self.radius as f64;
        let world_y = (v.1 as f64) * self.radius as f64;
        let world_z = (v.2 as f64) * self.radius as f64;

        let warped_x = world_x + (world_y * 0.15).sin() * 3.0;
        let warped_y = world_y + (world_z * 0.15).sin() * 3.0;
        let warped_z = world_z + (warped_x * 0.15).sin() * 3.0;

        let primary_noise = self.noise1.noise_3d_fbm(
            warped_x * freq_scale_x * 1.0,
            warped_y * freq_scale_y * 1.1,
            warped_z * freq_scale_z * 1.0,
            6,
            0.5,
            1.8,
        );

        let secondary_noise = self.noise2.noise_3d_fbm(
            warped_x * freq_scale_x * 1.3 + 0.5,
            warped_y * freq_scale_y * 2.8 + 0.2,
            warped_z * freq_scale_z * 1.3 + 0.7,
            3,
            0.5,
            2.0,
        ) * 2.0;

        let detail_noise = self.noise2.noise_3d_fbm(
            warped_x * freq_scale_x * 6.0,
            warped_y * freq_scale_y * 12.0,
            warped_z * freq_scale_z * 6.0,
            2,
            0.5,
            2.0,
        ) * 2.0;

        let blended_detail = lerp_nc(detail_noise, detail_noise * 0.1, self.mod_x);

        let reference_noise = self.noise2.noise_3d_fbm(
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
            f *= 2.0;
        }

        let mut t = levelize2(f, 1.0, 0.0);
        if t > 0.0 {
            let levelized_val = levelize2(f, 1.0, 0.0);
            t = lerp_nc(
                levelize4(levelized_val, 1.0, 0.0),
                levelized_val,
                self.mod_x,
            );
        }

        let height_b = if t > 0.0 {
            if t > 1.0 {
                if t > 2.0 {
                    lerp_nc(1.2, 2.0, t - 2.0) + blended_detail * 0.12
                } else {
                    lerp_nc(0.3, 1.2, t - 1.0) + blended_detail * 0.12
                }
            } else {
                lerp_nc(0.0, 0.3, t) + blended_detail * 0.1
            }
        } else {
            lerp_nc(-1.0, 0.0, t + 1.0)
        };

        let height_a2 = if t > 0.0 {
            if t > 1.0 {
                if t > 2.0 {
                    lerp_nc(1.4, 2.7, t - 2.0) + blended_detail * 0.12
                } else {
                    lerp_nc(0.3, 1.4, t - 1.0) + blended_detail * 0.12
                }
            } else {
                lerp_nc(0.0, 0.3, t) + blended_detail * 0.1
            }
        } else {
            lerp_nc(-4.0, 0.0, t + 1.0)
        };

        let final_height = lerp_nc(height_a2, height_b, self.mod_x);

        (self.radius as f64 + final_height + 0.2) as f32
    }
}
