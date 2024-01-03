use crate::data::enums::VeinType;
use crate::data::rule::Condition;
use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleAverageVeinAmount {
    pub vein: VeinType,
    pub condition: Condition,
}

impl Rule for RuleAverageVeinAmount {
    fn get_priority(&self) -> i32 {
        51
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            if !evaluation.is_unknown(index) {
                if !sp.is_safe() {
                    sp.load_planets();
                }
                continue;
            }
            let count = sp.get_avg_vein(&self.vein);
            if self.condition.eval(count) {
                result.push(index);
            }
        }
        result
    }
}
