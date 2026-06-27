use crate::data::enums::SpectrType;
use crate::data::galaxy::Galaxy;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_safe;
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

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        evaluate_safe!(galaxy, evaluation, |sp| {
            self.spectr.contains(&sp.star.get_spectr())
        })
    }
}
