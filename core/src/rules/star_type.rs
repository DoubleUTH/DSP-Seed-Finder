use crate::data::enums::StarType;
use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleStarType {
    pub star_type: Vec<StarType>,
}

impl Rule for RuleStarType {
    fn get_priority(&self) -> i32 {
        11
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
            if self.star_type.contains(&star.star_type) {
                result.push(index)
            }
        }
        result
    }
}
