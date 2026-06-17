use super::super::math::clamp01;
use super::super::planet::Planet;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;
use crate::data::planet_grid::{get_planet_grid, PlanetGrid};

/// PlanetAlgorithm12 - Latitude-based terrain with ridged noise and modX/modY.
pub struct PlanetAlgorithm12 {
    grid: &'static PlanetGrid,
    radius: f64,
    noise1: SimplexNoise,
    noise2: SimplexNoise,
    freq_scale: f64,
    mod_y: f64,
}

#[inline]
fn curve_evaluate(t: f64) -> f64 {
    let t = t / 0.6;
    if t >= 1.0 {
        0.0
    } else {
        (1.0 - t).powi(3) + (1.0 - t).powi(2) * 3.0 * t
    }
}

#[inline]
fn remap(src_min: f64, src_max: f64, tgt_min: f64, tgt_max: f64, x: f64) -> f64 {
    (x - src_min) / (src_max - src_min) * (tgt_max - tgt_min) + tgt_min
}

impl PlanetAlgorithm12 {
    pub fn new(planet: &Planet) -> Self {
        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        Self {
            grid: get_planet_grid(),
            radius: planet.radius as f64,
            noise1: SimplexNoise::with_seed(seed1),
            noise2: SimplexNoise::with_seed(seed2),
            freq_scale: 1.1 * planet.get_mod_x(),
            mod_y: planet.get_mod_y(),
        }
    }
}

impl PlanetAlgorithm for PlanetAlgorithm12 {
    fn get_height(&self, index: usize) -> f64 {
        let ridge_amplitude = 0.2;
        let height_multiplier = 8.0;
        let pi = std::f64::consts::PI;

        let v = self.grid.get_vertex(index);
        let latitude_factor = ((v.1 as f64).abs().asin()) * 2.0 / pi;
        let x_pos = v.0 as f64;
        let y_pos_mod = (v.1 as f64) * 2.5 * self.mod_y;
        let z_pos = v.2 as f64;

        let warp_offset = self.noise2.noise_3d_fbm(
            x_pos * self.freq_scale,
            y_pos_mod * self.freq_scale,
            z_pos * self.freq_scale,
            3,
            0.4,
            2.0,
        ) * 0.2;
        let ridged = self.noise1.ridged_noise(
            x_pos * self.freq_scale,
            y_pos_mod * self.freq_scale - warp_offset,
            z_pos * self.freq_scale,
            6,
            0.7,
            2.0,
            0.8,
        );
        let fbm_val = self.noise1.noise_3d_fbm_initial_amp(
            x_pos * self.freq_scale,
            y_pos_mod * self.freq_scale - warp_offset,
            z_pos * self.freq_scale,
            6,
            0.6,
            2.0,
            0.7,
        );
        let combined_noise = fbm_val * (ridged + fbm_val);

        let val = ((clamp01(remap(
            -8.0,
            8.0,
            0.0,
            1.0,
            ridge_amplitude + height_multiplier * combined_noise * ridged + 0.5,
        )) + 0.5)
            .powf(1.5)
            - curve_evaluate(latitude_factor * 0.9))
            * 2.0;

        let final_height = val.clamp(0.0, 2.0) * 1.1 - 0.2;

        self.radius + final_height
    }
}
