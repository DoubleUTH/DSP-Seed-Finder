use crate::data::rule::Condition;
use crate::data::rule::Rule;
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
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            let is_unknown = evaluation.is_unknonwn(index);
            let is_safe = sp.is_safe();
            if !is_unknown && is_safe {
                continue;
            }
            let mut total = 0.0;
            for planet in sp.get_planets() {
                if !planet.is_gas_giant() {
                    if !is_safe {
                        planet.get_theme();
                    }
                    continue;
                }
                for (gas_type, rate) in planet.get_gases() {
                    if *gas_type == self.gas_type {
                        total += *rate
                    }
                }
            }
            sp.mark_safe();
            if self.condition.eval(total) {
                result.push(index);
            }
        }
        result
    }
}
