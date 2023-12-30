use crate::data::rule::Condition;
use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleGasCount {
    pub cold: Option<bool>,
    pub condition: Condition,
}

impl Rule for RuleGasCount {
    fn get_priority(&self) -> i32 {
        if self.cold.is_some() {
            40
        } else {
            20
        }
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        if let Some(cold) = self.cold {
            for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
                let is_unknown = evaluation.is_unknonwn(index);
                if !is_unknown && sp.is_safe() {
                    continue;
                }
                let mut count = 0;
                for planet in sp.get_planets() {
                    let theme = planet.get_theme();
                    if planet.is_gas_giant() && (theme.temperature < 0.0) == cold {
                        count += 1;
                    }
                }
                sp.mark_safe();
                if self.condition.eval(count as f32) {
                    result.push(index);
                }
            }
        } else {
            for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
                if evaluation.is_known(index) {
                    continue;
                }
                let planets = sp.get_planets();
                let targets = planets
                    .iter()
                    .filter(|planet| planet.is_gas_giant())
                    .count();
                if self.condition.eval(targets as f32) {
                    result.push(index)
                }
            }
        }
        result
    }
}
