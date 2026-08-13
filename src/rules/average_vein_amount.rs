use crate::data::enums::VeinType;
use crate::data::galaxy::Galaxy;
use crate::data::rule::Condition;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_unsafe;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleAverageVeinAmount {
    pub use_actual: bool,
    pub vein: VeinType,
    pub condition: Condition,
}

impl Rule for RuleAverageVeinAmount {
    fn get_priority(&self) -> i32 {
        if self.use_actual {
            101
        } else {
            51
        }
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        evaluate_unsafe!(galaxy, evaluation, |sp| {
            if self.use_actual {
                // Actual-vein generation is ~340x the cost of the estimator
                // (terrain height per grid node). The estimated maxima give a
                // sound upper bound on the actual total, so most stars can be
                // decided without generating terrain.
                match self
                    .condition
                    .eval_interval(0.0, sp.get_max_possible_vein(&self.vein))
                {
                    Some(known) => known,
                    None => self.condition.eval(sp.get_actual_vein(&self.vein)),
                }
            } else {
                self.condition.eval(sp.get_avg_vein(&self.vein))
            }
        })
    }
}
