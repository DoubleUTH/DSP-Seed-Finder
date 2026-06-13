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

        let base_freq_x: f64 = 0.0035;
        let base_freq_y: f64 = 0.025 * mod_x_transformed + 0.0035 * (1.0 - mod_x_transformed);
        let base_freq_z: f64 = 0.0035;
        let noise_amplitude: f64 = 3.0;
        let mod_y_scale: f64 = 1.0 + 1.3 * mod_y;
        let scaled_freq_x: f64 = base_freq_x * mod_y_scale;
        let scaled_freq_y: f64 = base_freq_y * mod_y_scale;
        let scaled_freq_z: f64 = base_freq_z * mod_y_scale;

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let noise1 = SimplexNoise::with_seed(seed1);

        let data_length = planet_raw_data.data_length();
        let mut height_data = vec![0u16; data_length];
        let radius = planet.radius as f64;

        for i in 0..data_length {
            let v = &planet_raw_data.vertices[i];
            let world_x = (v.0 as f64) * radius;
            let world_y = (v.1 as f64) * radius;
            let world_z = (v.2 as f64) * radius;

            // First noise layer: 6 octaves, deltaAmp=0.45, deltaWlen=1.8
            let base_noise = noise1.noise_3d_fbm(
                world_x * scaled_freq_x,
                world_y * scaled_freq_y,
                world_z * scaled_freq_z,
                6,
                0.45,
                1.8,
            );

            // Terrain shaping
            let shaping_factor = noise_amplitude;
            let shaped_terrain =
                0.6 / ((base_noise * shaping_factor + shaping_factor * 0.4).abs() + 0.6) - 0.25;
            let final_height = if shaped_terrain < 0.0 {
                shaped_terrain * 0.3
            } else {
                shaped_terrain
            };

            // height = final_height (biomo calculations skipped)
            height_data[i] = ((radius + final_height + 0.1) * 100.0) as u16;
        }

        height_data
    }
}
