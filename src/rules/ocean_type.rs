use crate::data::galaxy::Galaxy;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_unsafe;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleOceanType {
    pub ocean_type: i32,
}

impl Rule for RuleOceanType {
    fn get_priority(&self) -> i32 {
        42
    }
    fn evaluate(
        &self,
        galaxy: &Galaxy,
        evaluation: &Evaluation,
    ) -> u64 {
        evaluate_unsafe!(galaxy, evaluation, |sp| {
            let mut found = false;
            for planet in sp.get_planets() {
                let theme = planet.get_theme();
                if self.ocean_type == theme.water_item_id {
                    found = true;
                }
            }
            found
        })
    }
}
