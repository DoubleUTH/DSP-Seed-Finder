use crate::data::rule::Condition;
use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleSatelliteCount {
    pub condition: Condition,
}

impl Rule for RuleSatelliteCount {
    fn get_priority(&self) -> i32 {
        31
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluation,
    ) -> u64 {
        let mut result: u64 = 0;
        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            if evaluation.is_known(index) {
                continue;
            }
            let planets = sp.get_planets();
            let targets = planets
                .iter()
                .filter(|planet| planet.has_orbit_around())
                .count();
            if self.condition.eval(targets as f32) {
                result |= 1 << index;
            }
        }
        result
    }
}
