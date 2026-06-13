use super::super::math::clamp01;
use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm12 - Latitude-based terrain with ridged noise and modX/modY.
pub struct PlanetAlgorithm12;

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

impl PlanetAlgorithm for PlanetAlgorithm12 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
        let mod_x = planet.get_mod_x();
        let mod_y = planet.get_mod_y();

        let num1 = 1.1 * mod_x;
        let num2 = 0.2;
        let num3 = 8.0;

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        let noise1 = SimplexNoise::with_seed(seed1);
        let noise2 = SimplexNoise::with_seed(seed2);

        let data_length = planet_raw_data.data_length();
        let mut height_data = vec![0u16; data_length];
        let radius = planet.radius as f64;
        let pi = std::f64::consts::PI;

        for i in 0..data_length {
            let v = &planet_raw_data.vertices[i];
            let num4 = ((v.1 as f64).abs().asin()) * 2.0 / pi;
            let x_pos = v.0 as f64;
            let num5 = (v.1 as f64) * 2.5 * mod_y;
            let z_pos = v.2 as f64;

            let num6 =
                noise2.noise_3d_fbm(x_pos * num1, num5 * num1, z_pos * num1, 3, 0.4, 2.0) * 0.2;
            let num7 = noise1.ridged_noise(
                x_pos * num1,
                num5 * num1 - num6,
                z_pos * num1,
                6,
                0.7,
                2.0,
                0.8,
            );
            let num8 = noise1.noise_3d_fbm_initial_amp(
                x_pos * num1,
                num5 * num1 - num6,
                z_pos * num1,
                6,
                0.6,
                2.0,
                0.7,
            );
            let num9 = num8 * (num7 + num8);

            let val = ((clamp01(remap(-8.0, 8.0, 0.0, 1.0, num2 + num3 * num9 * num7 + 0.5))
                + 0.5)
                .powf(1.5)
                - curve_evaluate(num4 * 0.9))
                * 2.0;

            let num10 = val.clamp(0.0, 2.0) * 1.1 - 0.2;

            height_data[i] = ((radius + num10) * 100.0) as u16;
        }

        height_data
    }
}
