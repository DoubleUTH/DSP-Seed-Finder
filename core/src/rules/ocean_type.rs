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
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            if !evaluation.is_unknown(index) {
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
                result.push(index);
            }
        }
        result
    }
}
