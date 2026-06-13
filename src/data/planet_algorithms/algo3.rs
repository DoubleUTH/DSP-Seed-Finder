use super::super::math::{levelize2, levelize4};
use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// Private helpers matching C# `Mathf.Lerp` double-precision behavior (no clamp).
#[inline]
fn lerp_nc(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// PlanetAlgorithm3 - Complex FBM noise with domain warping and multi-level shaping.
///
/// C# equivalent: PlanetAlgorithm3.cs
///
/// modX is a terrain modulation parameter (default 0.5) used for blending.
/// modY is not used in this algorithm.
pub struct PlanetAlgorithm3;

impl PlanetAlgorithm for PlanetAlgorithm3 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
        // Default modulation value (mid-range)
        let mod_x = planet.get_mod_x();

        let freq_scale_x: f64 = 0.007;
        let freq_scale_y: f64 = 0.007;
        let freq_scale_z: f64 = 0.007;

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

            // Domain warping: each axis is perturbed by sine of another axis
            let warped_x = world_x + (world_y * 0.15).sin() * 3.0;
            let warped_y = world_y + (world_z * 0.15).sin() * 3.0;
            let warped_z = world_z + (warped_x * 0.15).sin() * 3.0;

            // First noise layer: 6 octaves, deltaAmp=0.5, deltaWlen=1.8
            let primary_noise = noise1.noise_3d_fbm(
                warped_x * freq_scale_x * 1.0,
                warped_y * freq_scale_y * 1.1,
                warped_z * freq_scale_z * 1.0,
                6,
                0.5,
                1.8,
            );

            // Second noise layer: 3 octaves with offset coordinates
            let secondary_noise = noise2.noise_3d_fbm(
                warped_x * freq_scale_x * 1.3 + 0.5,
                warped_y * freq_scale_y * 2.8 + 0.2,
                warped_z * freq_scale_z * 1.3 + 0.7,
                3,
                0.5,
                2.0,
            ) * 2.0;

            // Detail noise (a): 2 octaves at high frequency
            let detail_noise = noise2.noise_3d_fbm(
                warped_x * freq_scale_x * 6.0,
                warped_y * freq_scale_y * 12.0,
                warped_z * freq_scale_z * 6.0,
                2,
                0.5,
                2.0,
            ) * 2.0;

            let blended_detail = lerp_nc(detail_noise, detail_noise * 0.1, mod_x);

            // Reference noise: 2 octaves
            let reference_noise = noise2.noise_3d_fbm(
                warped_x * freq_scale_x * 0.8,
                warped_y * freq_scale_y * 0.8,
                warped_z * freq_scale_z * 0.8,
                2,
                0.5,
                2.0,
            ) * 2.0;

            // Combine noise values
            let mut f = primary_noise * 2.0
                + 0.92
                + ((secondary_noise * (reference_noise + 0.5).abs() - 0.35) * 1.0).clamp(0.0, 1.0);

            if f < 0.0 {
                f *= 2.0;
            }

            // Levelize2 and blend
            let mut t = levelize2(f, 1.0, 0.0);
            if t > 0.0 {
                let levelized_val = levelize2(f, 1.0, 0.0);
                t = lerp_nc(levelize4(levelized_val, 1.0, 0.0), levelized_val, mod_x);
            }

            // Piecewise height calculation (b)
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

            // Piecewise height calculation (a2), then lerp with b
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

            let final_height = lerp_nc(height_a2, height_b, mod_x);

            // height = final_height (biomo calculations skipped)
            height_data[i] = ((radius + final_height + 0.2) * 100.0) as u16;
        }

        height_data
    }
}
