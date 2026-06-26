use crate::data::enums::VeinType;
use crate::data::rule::Condition;
use crate::data::rule::Rule;
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
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> u64 {
        let mut result: u64 = 0;
        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            if evaluation.is_known(index) {
                if !sp.is_safe() {
                    sp.load_planets();
                }
                continue;
            }
            let count = if self.use_actual {
                sp.get_actual_vein(&self.vein)
            } else {
                sp.get_avg_vein(&self.vein)
            };
            if self.condition.eval(count) {
                result |= 1 << index;
            }
        }
        result
    }
}
