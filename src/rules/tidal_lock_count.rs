use crate::data::galaxy::Galaxy;
use crate::data::rule::Condition;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_safe;
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

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        evaluate_safe!(galaxy, evaluation, |sp| {
            let targets = sp
                .get_planets()
                .iter()
                .filter(|planet| planet.is_tidal_locked())
                .count();
            self.condition.eval(targets as f32)
        })
    }
}
