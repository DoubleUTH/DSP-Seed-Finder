use super::super::planet::Planet;
use super::super::planet_raw_data::PlanetRawData;
use super::PlanetAlgorithm;

/// PlanetAlgorithm0 - All vertices at planet radius.
/// This is the simplest algorithm.
#[derive(Default)]
pub struct PlanetAlgorithm0 {
    radius: f32,
}

impl PlanetAlgorithm for PlanetAlgorithm0 {
    fn prepare_data(&mut self, planet: &Planet) {
        self.radius = planet.radius;
    }

    fn get_height(&self, _index: usize, _planet_raw_data: &PlanetRawData) -> f32 {
        self.radius
    }
}
