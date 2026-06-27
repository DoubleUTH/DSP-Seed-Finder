use crate::data::galaxy::Galaxy;
use crate::data::rule::Condition;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_safe;
use crate::evaluate_unsafe;
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

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        if let Some(ice) = self.ice {
            evaluate_unsafe!(galaxy, evaluation, |sp| {
                let mut count = 0;
                for planet in sp.get_planets() {
                    let theme = planet.get_theme();
                    if planet.is_gas_giant() && (theme.temperature < 0.0) == ice {
                        count += 1;
                    }
                }
                self.condition.eval(count as f32)
            })
        } else {
            evaluate_safe!(galaxy, evaluation, |sp| {
                let planets = sp.get_planets();
                let targets = planets
                    .iter()
                    .filter(|planet| planet.is_gas_giant())
                    .count();
                self.condition.eval(targets as f32)
            })
        }
    }
}
