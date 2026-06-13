use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm8 - Single noise layer with cosine-based terrain shaping.
#[derive(Default)]
pub struct PlanetAlgorithm8 {
    radius: f32,
    noise: Option<SimplexNoise>,
    freq_scale_x: f64,
    freq_scale_y: f64,
    freq_scale_z: f64,
    mod_y: f64,
}

impl PlanetAlgorithm for PlanetAlgorithm8 {
    fn prepare_data(&mut self, planet: &Planet) {
        self.radius = planet.radius;
        let mod_x = planet.get_mod_x();
        self.mod_y = planet.get_mod_y();

        self.freq_scale_x = 0.002 * mod_x;
        self.freq_scale_y = 0.002 * mod_x * mod_x * 6.66667;
        self.freq_scale_z = 0.002 * mod_x;

        self.noise = Some(SimplexNoise::with_seed(
            DspRandom::new(planet.seed).next_seed(),
        ));
    }

    fn get_height(&self, index: usize, planet_raw_data: &PlanetRawData) -> f32 {
        let v = &planet_raw_data.vertices[index];
        let world_x = (v.0 as f64) * self.radius as f64;
        let world_y = (v.1 as f64) * self.radius as f64;
        let world_z = (v.2 as f64) * self.radius as f64;

        let noise = self.noise.as_ref().unwrap();

        let noise_val = (noise.noise_3d_fbm(
            world_x * self.freq_scale_x,
            world_y * self.freq_scale_y,
            world_z * self.freq_scale_z,
            6,
            0.45,
            1.8,
        ) + 1.0
            + self.mod_y * 0.01)
            .clamp(0.0, 2.0);

        let shaped_height = if noise_val < 1.0 {
            let f = (noise_val * std::f64::consts::PI).cos() * 1.1;
            1.0 - ((f.signum() * f.powi(4)).clamp(-1.0, 1.0) + 1.0) * 0.5
        } else {
            let f = ((noise_val - 1.0) * std::f64::consts::PI).cos() * 1.1;
            2.0 - ((f.signum() * f.powi(4)).clamp(-1.0, 1.0) + 1.0) * 0.5
        };

        (self.radius as f64 + shaped_height + 0.1) as f32
    }
}
