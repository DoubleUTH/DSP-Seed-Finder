use super::star::Star;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Galaxy {
    pub seed: i32,
    pub stars: Vec<Star>,
}

impl Default for Galaxy {
    fn default() -> Self {
        Self {
            seed: 0,
            stars: vec![],
        }
    }
}

impl Galaxy {
    pub fn new() -> Self {
        Default::default()
    }
}
