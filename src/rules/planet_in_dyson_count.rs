use crate::data::galaxy::Galaxy;
use crate::data::rule::Condition;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_safe;
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

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        evaluate_safe!(galaxy, evaluation, |sp| {
            let planets = sp.get_planets();
            let dyson_radius = sp.star.get_dyson_radius() as f32;
            let targets = planets
                .iter()
                .filter(|planet| {
                    (self.include_giant || !planet.is_gas_giant())
                        && planet.get_sun_distance() * 40000.0 < dyson_radius
                })
                .count();
            self.condition.eval(targets as f32)
        })
    }
}
