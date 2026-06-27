use crate::data::galaxy::Galaxy;
use crate::data::rule::Condition;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_safe;
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

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        evaluate_safe!(galaxy, evaluation, |sp| {
            let planets = sp.get_planets();
            let targets = planets
                .iter()
                .filter(|planet| planet.has_orbit_around())
                .count();
            self.condition.eval(targets as f32)
        })
    }
}
