use crate::data::galaxy::Galaxy;
use crate::data::rule::Condition;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_safe;
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

    fn needs_walk(&self) -> bool {
        // The initial hive count depends on the star's safety factor, which is
        // derived from the star position's magnitude. The max hive count is not.
        self.initial
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        evaluate_safe!(galaxy, evaluation, |sp| {
            let count = if self.initial {
                sp.star.get_initial_hive_count()
            } else {
                sp.star.get_max_hive_count()
            };
            self.condition.eval(count as f32)
        })
    }
}
