use super::super::math::{levelize2, levelize3};
use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm11 - Complex noise with Remap, Levelize2/Levelize3 and modX/modY.
#[derive(Default)]
pub struct PlanetAlgorithm11 {
    radius: f32,
    mod_y: f64,
    noise1: Option<SimplexNoise>,
    noise2: Option<SimplexNoise>,
    noise3: Option<SimplexNoise>,
    mod_freq_x: f64,
    mod_freq_y: f64,
    mod_freq_z: f64,
}

#[inline]
fn remap(src_min: f64, src_max: f64, tgt_min: f64, tgt_max: f64, x: f64) -> f64 {
    (x - src_min) / (src_max - src_min) * (tgt_max - tgt_min) + tgt_min
}

impl PlanetAlgorithm for PlanetAlgorithm11 {
    fn prepare_data(&mut self, planet: &Planet) {
        self.radius = planet.radius;
        let mod_x = planet.get_mod_x();
        self.mod_y = planet.get_mod_y();

        self.mod_freq_x = 0.002 * mod_x;
        self.mod_freq_y = 0.002 * mod_x * 4.0;
        self.mod_freq_z = 0.002 * mod_x;

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        let seed3 = rand.next_seed();
        self.noise1 = Some(SimplexNoise::with_seed(seed1));
        self.noise2 = Some(SimplexNoise::with_seed(seed2));
        self.noise3 = Some(SimplexNoise::with_seed(seed3));
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

        let detail_noise = noise2.noise_3d_fbm(
            world_x * freq_scale_x * 4.0,
            world_y * freq_scale_y * 8.0,
            world_z * freq_scale_z * 4.0,
            3,
            0.5,
            2.0,
        );
        let primary_freq_scale = 0.6;

        let inner = noise1.noise_3d_fbm(
            world_x * freq_scale_x * primary_freq_scale,
            world_y * freq_scale_x * 1.5 * 2.5,
            world_z * freq_scale_x * primary_freq_scale,
            6,
            0.45,
            1.8,
        ) * 0.95
            + detail_noise * 0.05;

        let primary_shaped = levelize2(
            (remap(-1.0, 1.0, 0.0, 1.0, inner)).powf(self.mod_y) + 1.0,
            1.0,
            0.0,
        );

        let inner2 = noise3.noise_3d_fbm(
            world_x * self.mod_freq_x,
            world_y * self.mod_freq_y,
            world_z * self.mod_freq_z,
            5,
            0.55,
            2.0,
        );
        let secondary_shaped =
            levelize3((remap(-1.0, 1.0, 0.0, 1.0, inner2)).powf(0.65), 1.0, 0.0) * primary_shaped;

        let final_height = ((secondary_shaped - 0.4) * 0.9).max(-0.3);

        (self.radius as f64 + final_height) as f32
    }
}
