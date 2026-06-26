use crate::data::planet::Planet;
use crate::data::planet_algorithms::{create_and_prepare_algo, PlanetAlgorithm};
use crate::data::planet_grid::{
    get_planet_grid, position_hash, PlanetGrid, DATA_LENGTH, PRECISION, STRIDE,
};

use super::vector_f3::VectorF3;
use std::f64::consts::PI;

pub struct PlanetRawData {
    grid: &'static PlanetGrid,
    algo: Box<dyn PlanetAlgorithm>,
    cache: Vec<f32>,
}

impl PlanetRawData {
    pub fn new(planet: &Planet) -> Self {
        Self {
            grid: get_planet_grid(),
            algo: create_and_prepare_algo(planet),
            cache: vec![f32::NAN; DATA_LENGTH],
        }
    }

    #[inline]
    fn get_height(&mut self, index: usize) -> f32 {
        let val = self.cache[index];
        if val.is_nan() {
            let h = (self.algo.get_height(index) * 100.0) as u16 as f32;
            self.cache[index] = h;
            h
        } else {
            val
        }
    }

    pub fn query_height_normalized(&mut self, vpos_normalized: &VectorF3) -> f32 {
        let index1 = self.grid.index_map[position_hash(vpos_normalized, 0)];

        let num1: f64 = (PI / (PRECISION as f64 * 2.0)) * 1.2_f64;
        let num2: f64 = num1 * num1;

        let mut num3: f32 = 0.0f32;
        let mut num4: f32 = 0.0f32;

        for i3 in -1..=3 {
            let i4 = index1 + i3 * STRIDE;
            for i2 in -1_i32..=3 {
                let idx4 = (i4 + i2) as usize;
                if idx4 < DATA_LENGTH {
                    let sqr_mag = self.grid.vertices[idx4].distance_sq_from(vpos_normalized);
                    if (sqr_mag as f64) <= num2 {
                        let num5 = 1.0f32 - (sqr_mag.sqrt() / num1 as f32);
                        let num6 = self.get_height(idx4);
                        num3 += num5;
                        num4 += num6 * num5;
                    }
                }
            }
        }

        if num3 != 0.0f32 {
            num4 / num3 * 0.01
        } else {
            self.get_height(0) * 0.01
        }
    }

    #[inline]
    pub fn query_height(&mut self, vpos: &VectorF3) -> f32 {
        let mut vpos = *vpos;
        vpos.normalize();
        self.query_height_normalized(&vpos)
    }
}
