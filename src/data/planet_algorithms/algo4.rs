use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::super::random::DspRandom;
use super::super::random_table::RandomTable;
use super::super::simplex_noise::SimplexNoise;
use super::PlanetAlgorithm;

/// PlanetAlgorithm4 - FBM noise with 80 circular crater features.
#[derive(Default)]
pub struct PlanetAlgorithm4 {
    radius: f32,
    noise1: Option<SimplexNoise>,
    noise2: Option<SimplexNoise>,
    circles: Option<Vec<([f64; 3], f64)>>,
    heights: Option<Vec<f64>>,
}

impl PlanetAlgorithm for PlanetAlgorithm4 {
    fn prepare_data(&mut self, planet: &Planet) {
        self.radius = planet.radius;

        let mut rand = DspRandom::new(planet.seed);
        let seed1 = rand.next_seed();
        let seed2 = rand.next_seed();
        self.noise1 = Some(SimplexNoise::with_seed(seed1));
        self.noise2 = Some(SimplexNoise::with_seed(seed2));

        let mut seed3 = rand.next_seed();
        let mut cr = Vec::with_capacity(80);
        let mut hs = Vec::with_capacity(80);

        for _ in 0..80 {
            let mut v = RandomTable::spheric_normal(&mut seed3, 1.0);
            let w = (v.magnitude() * 8.0 + 8.0) as f32;
            v.normalize();
            v *= planet.radius as f64;
            cr.push(([v.0, v.1, v.2], (w * w) as f64));

            let h = rand.next_f64() * 0.4 + 0.2;
            hs.push(h);
        }

        self.circles = Some(cr);
        self.heights = Some(hs);
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

        let low_freq_noise = noise1.noise_3d_fbm(
            world_x * freq_scale_x,
            world_y * freq_scale_y,
            world_z * freq_scale_z,
            4,
            0.45,
            1.8,
        );
        let high_freq_noise = noise2.noise_3d_fbm(
            world_x * freq_scale_x * 5.0,
            world_y * freq_scale_y * 5.0,
            world_z * freq_scale_z * 5.0,
            4,
            0.5,
            2.0,
        );

        let scaled_low = low_freq_noise * 1.5;
        let scaled_high = high_freq_noise * 0.2;
        let base_elevation = scaled_low * 0.08 + scaled_high * 2.0;

        let mut max_crater = 0.0;
        let circles = self.circles.as_ref().unwrap();
        let heights = self.heights.as_ref().unwrap();
        for j in 0..80 {
            let c = &circles[j];
            let dx = c.0[0] - world_x;
            let dy = c.0[1] - world_y;
            let dz = c.0[2] - world_z;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            if dist_sq <= c.1 {
                let mut t = dist_sq / c.1 + scaled_high * 1.2;
                if t < 0.0 {
                    t = 0.0;
                }
                let t_sq = t * t;
                let crater_shape = -15.0 * (t_sq * t) + (131.0 / 6.0) * t_sq - (113.0 / 15.0) * t
                    + 0.7
                    + scaled_high;
                let crater_shape = if crater_shape < 0.0 {
                    0.0
                } else {
                    crater_shape
                };
                let crater_val = crater_shape * crater_shape * heights[j];
                if crater_val > max_crater {
                    max_crater = crater_val;
                }
            }
        }

        let final_height = max_crater + base_elevation + 0.2;
        (self.radius as f64 + final_height + 0.1) as f32
    }
}
