use crate::data::galaxy::Galaxy;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleBirth {}

impl Rule for RuleBirth {
    fn get_priority(&self) -> i32 {
        10
    }
    fn evaluate(&self, _: &Galaxy, _: &Evaluation) -> u64 {
        1
    }
}
