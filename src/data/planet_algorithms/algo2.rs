use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm2 - Two-layer FBM noise with frequency modulation from modX/modY.
#[derive(Default)]
pub struct PlanetAlgorithm2 {
    radius: f32,
    noise1: Option<SimplexNoise>,
    /// Pre-computed scaling constants
    scaled_freq_x: f64,
    scaled_freq_y: f64,
    scaled_freq_z: f64,
}

impl PlanetAlgorithm for PlanetAlgorithm2 {
    fn prepare_data(&mut self, planet: &Planet) {
        self.radius = planet.radius;
        let mod_x = planet.get_mod_x();
        let mod_y = planet.get_mod_y();

        let mod_x_transformed = (3.0 - mod_x - mod_x) * mod_x * mod_x;

        let base_freq_x: f64 = 0.0035;
        let base_freq_y: f64 = 0.025 * mod_x_transformed + 0.0035 * (1.0 - mod_x_transformed);
        let base_freq_z: f64 = 0.0035;
        let mod_y_scale: f64 = 1.0 + 1.3 * mod_y;
        self.scaled_freq_x = base_freq_x * mod_y_scale;
        self.scaled_freq_y = base_freq_y * mod_y_scale;
        self.scaled_freq_z = base_freq_z * mod_y_scale;

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        self.noise1 = Some(SimplexNoise::with_seed(seed1));
    }

    fn get_height(&self, index: usize, planet_raw_data: &PlanetRawData) -> f32 {
        let noise_amplitude: f64 = 3.0;

        let v = &planet_raw_data.vertices[index];
        let world_x = (v.0 as f64) * self.radius as f64;
        let world_y = (v.1 as f64) * self.radius as f64;
        let world_z = (v.2 as f64) * self.radius as f64;

        let noise1 = self.noise1.as_ref().unwrap();

        let base_noise = noise1.noise_3d_fbm(
            world_x * self.scaled_freq_x,
            world_y * self.scaled_freq_y,
            world_z * self.scaled_freq_z,
            6,
            0.45,
            1.8,
        );

        let shaping_factor = noise_amplitude;
        let shaped_terrain =
            0.6 / ((base_noise * shaping_factor + shaping_factor * 0.4).abs() + 0.6) - 0.25;
        let final_height = if shaped_terrain < 0.0 {
            shaped_terrain * 0.3
        } else {
            shaped_terrain
        };

        (self.radius as f64 + final_height + 0.1) as f32
    }
}
