use crate::data::rule::Rule;
use crate::data::star::Star;
use crate::data::{planet::Planet, rule::Condition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RulePlanetCount {
    #[serde(skip)]
    pub evaluated: bool,
    pub condition: Condition,
}

impl Rule for RulePlanetCount {
    fn on_planets_created(&mut self, _: &Star, planets: &Vec<Planet>) -> Option<bool> {
        self.evaluated = true;
        Some(self.condition.eval(planets.len() as f32))
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
