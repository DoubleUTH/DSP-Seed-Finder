use super::star_planets::StarWithPlanets;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Galaxy<'a> {
    pub seed: i32,
    pub stars: Vec<StarWithPlanets<'a>>,
}

impl Default for Galaxy<'_> {
    fn default() -> Self {
        Self {
            seed: 0,
            stars: vec![],
        }
    }
}

impl Galaxy<'_> {
    pub fn new() -> Self {
        Default::default()
    }
}
