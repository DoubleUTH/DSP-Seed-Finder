use super::star::Star;

#[derive(Debug, Clone)]
pub struct Galaxy {
    pub seed: i32,
    pub star_count: i32,
    pub stars: Vec<Star>,
    pub birth_planet_id: i32,
    pub habitable_count: i32,
}

impl Default for Galaxy {
    fn default() -> Self {
        Self {
            seed: 0,
            star_count: 0,
            stars: vec![],
            birth_planet_id: 0,
            habitable_count: 0,
        }
    }
}

impl Galaxy {
    pub fn new() -> Self {
        Default::default()
    }
}
