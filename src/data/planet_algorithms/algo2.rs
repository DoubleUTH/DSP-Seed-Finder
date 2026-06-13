use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm2 - Two-layer FBM noise with frequency modulation from modX/modY.
///
/// C# equivalent: PlanetAlgorithm2.cs
///
/// modX and modY are terrain modulation parameters (default 0.5).
/// modX controls the blend between two noise frequency values.
/// modY controls overall amplitude scaling.
pub struct PlanetAlgorithm2;

impl PlanetAlgorithm for PlanetAlgorithm2 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
        // Default modulation values (mid-range). In the original C# these are
        // passed as parameters to GenerateTerrain(modX, modY).
        let mod_x = planet.get_mod_x();
        let mod_y = planet.get_mod_y();

        // modX transformation: (3 - 2*modX) * modX^2
        let mod_x_transformed = (3.0 - mod_x - mod_x) * mod_x * mod_x;

        let num1: f64 = 0.0035;
        let num2: f64 = 0.025 * mod_x_transformed + 0.0035 * (1.0 - mod_x_transformed);
        let num3: f64 = 0.0035;
        let num4: f64 = 3.0;
        let num5: f64 = 1.0 + 1.3 * mod_y;
        let num6: f64 = num1 * num5;
        let num7: f64 = num2 * num5;
        let num8: f64 = num3 * num5;

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let noise1 = SimplexNoise::with_seed(seed1);

        let data_length = planet_raw_data.data_length();
        let mut height_data = vec![0u16; data_length];
        let radius = planet.radius as f64;

        for i in 0..data_length {
            let v = &planet_raw_data.vertices[i];
            let num9 = (v.0 as f64) * radius;
            let num10 = (v.1 as f64) * radius;
            let num11 = (v.2 as f64) * radius;

            // First noise layer: 6 octaves, deltaAmp=0.45, deltaWlen=1.8
            let num12 = noise1.noise_3d_fbm(num9 * num6, num10 * num7, num11 * num8, 6, 0.45, 1.8);

            // Terrain shaping
            let num14 = num4;
            let num15 = 0.6 / ((num12 * num14 + num14 * 0.4).abs() + 0.6) - 0.25;
            let num16 = if num15 < 0.0 { num15 * 0.3 } else { num15 };

            // height = num16 (biomo calculations skipped)
            height_data[i] = ((radius + num16 + 0.1) * 100.0) as u16;
        }

        height_data
    }
}
