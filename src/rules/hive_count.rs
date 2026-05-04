use crate::data::rule::{Condition, Rule};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleHiveCount {
    pub condition: Condition,
    pub initial: bool,
}

impl Rule for RuleHiveCount {
    fn get_priority(&self) -> i32 {
        if self.initial {
            23
        } else {
            13
        }
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
            let star = &sp.star;
            let count = if self.initial {
                star.get_initial_hive_count()
            } else {
                star.get_max_hive_count()
            };
            if self.condition.eval(count as f32) {
                result.push(index)
            }
        }
        result
    }
}
