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
        40
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            let is_unknown = evaluation.is_unknonwn(index);
            if !is_unknown && sp.is_safe() {
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
