use crate::data::galaxy::Galaxy;
use crate::data::rule::Condition;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_safe;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RulePlanetCount {
    #[serde(default)]
    pub exclude_giant: bool,
    pub condition: Condition,
}

impl Rule for RulePlanetCount {
    fn get_priority(&self) -> i32 {
        30
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        evaluate_safe!(galaxy, evaluation, |sp| {
            let planets = sp.get_planets();
            let len = if self.exclude_giant {
                planets.iter().filter(|p| !p.is_gas_giant()).count()
            } else {
                planets.len()
            };
            self.condition.eval(len as f32)
        })
    }
}
