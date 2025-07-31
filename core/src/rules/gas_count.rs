use crate::data::rule::Condition;
use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleGasCount {
    #[serde(default)]
    pub ice: Option<bool>,
    pub condition: Condition,
}

impl Rule for RuleGasCount {
    fn get_priority(&self) -> i32 {
        if self.ice.is_some() {
            41
        } else {
            32
        }
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        if let Some(ice) = self.ice {
            for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
                let is_safe = sp.is_safe();
                if !evaluation.is_unknown(index) {
                    if !is_safe {
                        sp.load_planets()
                    }
                    continue;
                }
                let mut count = 0;
                for planet in sp.get_planets() {
                    if !planet.is_gas_giant() {
                        if !is_safe {
                            planet.get_theme();
                        }
                        continue;
                    }
                    let theme = planet.get_theme();
                    if (theme.temperature < 0.0) == ice {
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
