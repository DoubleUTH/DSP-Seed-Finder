use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleBirth {}

impl Rule for RuleBirth {
    fn get_priority(&self) -> i32 {
        10
    }
    fn evaluate(
        &self,
        _: &crate::data::galaxy::Galaxy,
        _: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        vec![0]
    }
}
