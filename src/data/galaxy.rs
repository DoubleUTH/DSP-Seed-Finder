use super::star_planets::StarWithPlanets;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Galaxy<'a> {
    pub seed: i32,
    pub stars: Vec<StarWithPlanets<'a>>,
}
