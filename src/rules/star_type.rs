use crate::data::enums::StarType;
use crate::data::galaxy::Galaxy;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_safe;
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

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        evaluate_safe!(galaxy, evaluation, |sp| {
            self.star_type.contains(&sp.star.star_type)
        })
    }
}
