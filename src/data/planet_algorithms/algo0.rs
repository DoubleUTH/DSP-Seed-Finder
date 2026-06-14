use super::super::planet::Planet;
use super::PlanetAlgorithm;

/// PlanetAlgorithm0 - All vertices at planet radius.
/// This is the simplest algorithm.
pub struct PlanetAlgorithm0 {
    radius: f64,
}

impl PlanetAlgorithm0 {
    pub fn new(planet: &Planet) -> Self {
        Self {
            radius: planet.radius as f64,
        }
    }
}

impl PlanetAlgorithm for PlanetAlgorithm0 {
    fn get_height(&self, _index: usize) -> f64 {
        self.radius
    }
}
