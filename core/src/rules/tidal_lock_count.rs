use crate::data::rule::Condition;
use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleTidalLockCount {
    pub condition: Condition,
}

impl Rule for RuleTidalLockCount {
    fn get_priority(&self) -> i32 {
        33
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
            let targets = planets
                .iter()
                .filter(|planet| planet.is_tidal_locked())
                .count();
            if self.condition.eval(targets as f32) {
                result.push(index)
            }
        }
        result
    }
}
