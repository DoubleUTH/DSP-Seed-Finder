use crate::data::planet::Planet;
use crate::data::rule::{Condition, Rule};
use crate::data::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleLuminosity {
    #[serde(skip)]
    pub evaluated: bool,
    pub condition: Condition,
}

impl Rule for RuleLuminosity {
    fn on_planets_created(&mut self, star: &Star, _: &Vec<Planet>) -> Option<bool> {
        self.evaluated = true;
        Some(self.condition.eval(star.get_luminosity()))
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
