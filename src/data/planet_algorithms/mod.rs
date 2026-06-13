/// Planet algorithm implementations ported from PlanetAlgorithm0 through PlanetAlgorithm14.
/// Each algorithm generates height data for a planet based on its seed and theme.
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

/// Trait for planet algorithms. Each algorithm implements GenerateTerrain to produce
/// height data for a planet.
pub trait PlanetAlgorithm {
    /// Generate the terrain height data.
    ///
    /// # Arguments
    /// * `planet` - The planet configuration (contains seed, theme, etc.)
    /// * `planet_raw_data` - Mutable raw data buffer (use data.mod_* to access/modify)
    ///
    /// # Returns
    /// A `Vec<u16>` containing the height data for each vertex.
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16>;
}

/// Dispatch to the correct algorithm based on the algorithm ID stored in the planet.
pub fn generate_terrain(planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
    let algo_id = planet.get_algo_id();
    match algo_id {
        0 => PlanetAlgorithm0.generate_terrain(planet, planet_raw_data),
        1 => PlanetAlgorithm1.generate_terrain(planet, planet_raw_data),
        2 => PlanetAlgorithm2.generate_terrain(planet, planet_raw_data),
        3 => PlanetAlgorithm3.generate_terrain(planet, planet_raw_data),
        4 => PlanetAlgorithm4.generate_terrain(planet, planet_raw_data),
        5 => PlanetAlgorithm5.generate_terrain(planet, planet_raw_data),
        6 => PlanetAlgorithm6.generate_terrain(planet, planet_raw_data),
        7 => PlanetAlgorithm7.generate_terrain(planet, planet_raw_data),
        8 => PlanetAlgorithm8.generate_terrain(planet, planet_raw_data),
        9 => PlanetAlgorithm9.generate_terrain(planet, planet_raw_data),
        10 => PlanetAlgorithm10.generate_terrain(planet, planet_raw_data),
        11 => PlanetAlgorithm11.generate_terrain(planet, planet_raw_data),
        12 => PlanetAlgorithm12.generate_terrain(planet, planet_raw_data),
        13 => PlanetAlgorithm13.generate_terrain(planet, planet_raw_data),
        14 => PlanetAlgorithm14.generate_terrain(planet, planet_raw_data),
        _ => panic!("Unknown planet algorithm ID: {}", algo_id),
        // _ => PlanetAlgorithm0.generate_terrain(planet, planet_raw_data),
    }
}
