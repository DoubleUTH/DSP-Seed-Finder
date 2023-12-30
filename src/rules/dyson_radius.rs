use crate::data::rule::{Condition, Rule};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleDysonRadius {
    pub condition: Condition,
}

impl Rule for RuleDysonRadius {
    fn get_priority(&self) -> i32 {
        10
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
            if self.condition.eval(star.get_dyson_radius()) {
                result.push(index)
            }
        }
        result
    }
}
