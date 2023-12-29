use crate::data::planet::Planet;
use crate::data::rule::Rule;
use crate::data::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleOceanType {
    #[serde(skip)]
    pub evaluated: bool,
    pub ocean_type: Vec<i32>,
}

impl Rule for RuleOceanType {
    fn on_planets_themed(&mut self, _: &Star, planets: &Vec<Planet>) -> Option<bool> {
        self.evaluated = true;
        Some(
            self.ocean_type
                .iter()
                .all(|t| planets.iter().any(|p| *t == p.get_theme().water_item_id)),
        )
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
