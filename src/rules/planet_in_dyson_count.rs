use crate::data::rule::{Condition, Rule};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RulePlanetInDysonCount {
    pub include_giant: bool,
    pub condition: Condition,
}

impl Rule for RulePlanetInDysonCount {
    fn get_priority(&self) -> i32 {
        34
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            if evaluation.is_known(index) {
                continue;
            }
            let planets = sp.get_planets();
            let dyson_radius = sp.star.get_dyson_radius() as f32;
            let targets = planets
                .filter(|planet| {
                    (self.include_giant || !planet.is_gas_giant())
                        && planet.get_sun_distance() * 40000.0 < dyson_radius
                })
                .count();
            if self.condition.eval(targets as f32) {
                result.push(index)
            }
        }
        result
    }
}
