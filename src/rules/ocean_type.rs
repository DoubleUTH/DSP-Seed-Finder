use crate::data::rule::Rule;
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
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluation,
    ) -> u64 {
        let mut result: u64 = 0;
        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            if evaluation.is_known(index) {
                if !sp.is_safe() {
                    sp.load_planets()
                }
                continue;
            }
            let mut found = false;
            for planet in sp.get_planets() {
                let theme = planet.get_theme();
                if self.ocean_type == theme.water_item_id {
                    found = true;
                    // cannot early break because it is not safe
                }
            }
            sp.mark_safe();
            if found {
                result |= 1 << index;
            }
        }
        result
    }
}
