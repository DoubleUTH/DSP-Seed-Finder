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
            let count = if self.use_actual {
                sp.get_actual_vein(&self.vein)
            } else {
                sp.get_avg_vein(&self.vein)
            };
            self.condition.eval(count)
        })
    }
}
