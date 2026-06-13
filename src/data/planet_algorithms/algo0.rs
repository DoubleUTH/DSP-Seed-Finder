use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::PlanetAlgorithm;

/// PlanetAlgorithm0 - All vertices at planet radius * 100.
/// This is the simplest algorithm.
pub struct PlanetAlgorithm0;

impl PlanetAlgorithm for PlanetAlgorithm0 {
    fn generate_terrain(&self, planet: &Planet, planet_raw_data: &PlanetRawData) -> Vec<u16> {
        let data_length = planet_raw_data.data_length();
        let radius_100 = (planet.radius * 100.0) as u16;
        vec![radius_100; data_length]
    }
}
