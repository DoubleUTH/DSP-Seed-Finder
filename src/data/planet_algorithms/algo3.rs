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

        let num1: f64 = 0.007;
        let num2: f64 = 0.007;
        let num3: f64 = 0.007;

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
            let num4 = (v.0 as f64) * radius;
            let num5 = (v.1 as f64) * radius;
            let num6 = (v.2 as f64) * radius;

            // Domain warping: each axis is perturbed by sine of another axis
            let num7 = num4 + (num5 * 0.15).sin() * 3.0;
            let num8 = num5 + (num6 * 0.15).sin() * 3.0;
            let num9 = num6 + (num7 * 0.15).sin() * 3.0;

            // First noise layer: 6 octaves, deltaAmp=0.5, deltaWlen=1.8
            let num10 = noise1.noise_3d_fbm(
                num7 * num1 * 1.0,
                num8 * num2 * 1.1,
                num9 * num3 * 1.0,
                6,
                0.5,
                1.8,
            );

            // Second noise layer: 3 octaves with offset coordinates
            let num11 = noise2.noise_3d_fbm(
                num7 * num1 * 1.3 + 0.5,
                num8 * num2 * 2.8 + 0.2,
                num9 * num3 * 1.3 + 0.7,
                3,
                0.5,
                2.0,
            ) * 2.0;

            // Detail noise (a): 2 octaves at high frequency
            let a_val = noise2.noise_3d_fbm(
                num7 * num1 * 6.0,
                num8 * num2 * 12.0,
                num9 * num3 * 6.0,
                2,
                0.5,
                2.0,
            ) * 2.0;

            let num12 = lerp_nc(a_val, a_val * 0.1, mod_x);

            // Reference noise (num13): 2 octaves
            let num13 = noise2.noise_3d_fbm(
                num7 * num1 * 0.8,
                num8 * num2 * 0.8,
                num9 * num3 * 0.8,
                2,
                0.5,
                2.0,
            ) * 2.0;

            // Combine noise values
            let mut f =
                num10 * 2.0 + 0.92 + ((num11 * (num13 + 0.5).abs() - 0.35) * 1.0).clamp(0.0, 1.0);

            if f < 0.0 {
                f *= 2.0;
            }

            // Levelize2 and blend
            let mut t = levelize2(f, 1.0, 0.0);
            if t > 0.0 {
                let num14 = levelize2(f, 1.0, 0.0);
                t = lerp_nc(levelize4(num14, 1.0, 0.0), num14, mod_x);
            }

            // Piecewise height calculation (b)
            let b = if t > 0.0 {
                if t > 1.0 {
                    if t > 2.0 {
                        lerp_nc(1.2, 2.0, t - 2.0) + num12 * 0.12
                    } else {
                        lerp_nc(0.3, 1.2, t - 1.0) + num12 * 0.12
                    }
                } else {
                    lerp_nc(0.0, 0.3, t) + num12 * 0.1
                }
            } else {
                lerp_nc(-1.0, 0.0, t + 1.0)
            };

            // Piecewise height calculation (a2), then lerp with b
            let a2 = if t > 0.0 {
                if t > 1.0 {
                    if t > 2.0 {
                        lerp_nc(1.4, 2.7, t - 2.0) + num12 * 0.12
                    } else {
                        lerp_nc(0.3, 1.4, t - 1.0) + num12 * 0.12
                    }
                } else {
                    lerp_nc(0.0, 0.3, t) + num12 * 0.1
                }
            } else {
                lerp_nc(-4.0, 0.0, t + 1.0)
            };

            let num15 = lerp_nc(a2, b, mod_x);

            // height = num15 (biomo calculations skipped)
            height_data[i] = ((radius + num15 + 0.2) * 100.0) as u16;
        }

        height_data
    }
}
