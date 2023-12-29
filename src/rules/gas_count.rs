use crate::data::planet::Planet;
use crate::data::rule::{Condition, Rule};
use crate::data::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleGasCount {
    #[serde(skip)]
    pub evaluated: bool,
    pub cold: Option<bool>,
    pub condition: Condition,
}

impl Rule for RuleGasCount {
    fn on_planets_created(&mut self, _: &Star, planets: &Vec<Planet>) -> Option<bool> {
        if self.cold == None {
            self.evaluated = true;
            let count = planets
                .iter()
                .filter(|planet| planet.is_gas_giant())
                .count();
            Some(self.condition.eval(count as f32))
        } else {
            None
        }
    }
    fn on_planets_themed(&mut self, _: &Star, planets: &Vec<Planet>) -> Option<bool> {
        self.evaluated = true;
        let cold = self.cold.unwrap();
        let count = planets
            .iter()
            .filter(|planet| {
                planet.is_gas_giant() && cold == (planet.get_theme().temperature < 0.0)
            })
            .count();
        Some(self.condition.eval(count as f32))
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
