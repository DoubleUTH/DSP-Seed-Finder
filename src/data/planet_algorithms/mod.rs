/// Planet algorithm implementations ported from PlanetAlgorithm0 through PlanetAlgorithm14.
/// Each algorithm lazily computes height data for a planet based on its seed and theme.
use super::planet::Planet;
use super::planet_raw_data::PlanetRawData;

use algo0::PlanetAlgorithm0;
use algo1::PlanetAlgorithm1;
use algo10::PlanetAlgorithm10;
use algo11::PlanetAlgorithm11;
use algo12::PlanetAlgorithm12;
use algo13::PlanetAlgorithm13;
use algo14::PlanetAlgorithm14;
use algo2::PlanetAlgorithm2;
use algo3::PlanetAlgorithm3;
use algo4::PlanetAlgorithm4;
use algo5::PlanetAlgorithm5;
use algo6::PlanetAlgorithm6;
use algo7::PlanetAlgorithm7;
use algo8::PlanetAlgorithm8;
use algo9::PlanetAlgorithm9;

mod algo0;
mod algo1;
mod algo10;
mod algo11;
mod algo12;
mod algo13;
mod algo14;
mod algo2;
mod algo3;
mod algo4;
mod algo5;
mod algo6;
mod algo7;
mod algo8;
mod algo9;

/// Trait for planet algorithms. Each algorithm lazily computes height for individual vertices.
pub trait PlanetAlgorithm {
    /// Prepare any per-planet state (seeds, noise generators, radius, etc.).
    /// Called once before any calls to `get_height`.
    fn prepare_data(&mut self, planet: &Planet);

    /// Compute the height for a single vertex index.
    ///
    /// # Arguments
    /// * `index` - The vertex index (0..planet_raw_data.data_length())
    /// * `planet_raw_data` - Provides the vertex positions
    ///
    /// # Returns
    /// Height in game units (f32).
    fn get_height(&self, index: usize, planet_raw_data: &PlanetRawData) -> f32;
}

/// Construct and prepare the algorithm matching the planet's algo ID.
/// Returns a boxed, prepared algorithm ready for lazy height queries.
pub fn create_and_prepare_algo(planet: &Planet) -> Box<dyn PlanetAlgorithm> {
    let algo_id = planet.get_algo_id();
    match algo_id {
        0 => new_and_prepare(PlanetAlgorithm0::default(), planet),
        1 => new_and_prepare(PlanetAlgorithm1::default(), planet),
        2 => new_and_prepare(PlanetAlgorithm2::default(), planet),
        3 => new_and_prepare(PlanetAlgorithm3::default(), planet),
        4 => new_and_prepare(PlanetAlgorithm4::default(), planet),
        5 => new_and_prepare(PlanetAlgorithm5::default(), planet),
        6 => new_and_prepare(PlanetAlgorithm6::default(), planet),
        7 => new_and_prepare(PlanetAlgorithm7::default(), planet),
        8 => new_and_prepare(PlanetAlgorithm8::default(), planet),
        9 => new_and_prepare(PlanetAlgorithm9::default(), planet),
        10 => new_and_prepare(PlanetAlgorithm10::default(), planet),
        11 => new_and_prepare(PlanetAlgorithm11::default(), planet),
        12 => new_and_prepare(PlanetAlgorithm12::default(), planet),
        13 => new_and_prepare(PlanetAlgorithm13::default(), planet),
        14 => new_and_prepare(PlanetAlgorithm14::default(), planet),
        _ => panic!("Unknown planet algorithm ID: {}", algo_id),
    }
}

fn new_and_prepare<T: PlanetAlgorithm + Default + 'static>(
    mut algo: T,
    planet: &Planet,
) -> Box<dyn PlanetAlgorithm> {
    algo.prepare_data(planet);
    Box::new(algo)
}
