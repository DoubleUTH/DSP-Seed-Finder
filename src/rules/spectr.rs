use crate::data::enums::SpectrType;
use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleSpectr {
    pub spectr: Vec<SpectrType>,
}

impl Rule for RuleSpectr {
    fn get_priority(&self) -> i32 {
        21
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
            if self.spectr.contains(&star.get_spectr()) {
                result.push(index)
            }
        }
        result
    }
}
