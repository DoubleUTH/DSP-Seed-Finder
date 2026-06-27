use crate::data::galaxy::Galaxy;
use crate::data::rule::Condition;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_unsafe;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleGasRate {
    pub gas_type: i32,
    pub condition: Condition,
}

impl Rule for RuleGasRate {
    fn get_priority(&self) -> i32 {
        50
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        evaluate_unsafe!(galaxy, evaluation, |sp| {
            let mut total = 0.0;
            for planet in sp.get_planets() {
                if !planet.is_gas_giant() {
                    planet.get_theme();
                    continue;
                }
                for (gas_type, rate) in planet.get_gases() {
                    if *gas_type == self.gas_type {
                        total += *rate
                    }
                }
            }
            self.condition.eval(total)
        })
    }
}
