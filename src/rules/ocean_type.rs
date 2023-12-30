use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleOceanType {
    pub ocean_type: i32,
}

impl Rule for RuleOceanType {
    fn get_priority(&self) -> i32 {
        40
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            let is_unknown = evaluation.is_unknonwn(index);
            if !is_unknown && sp.is_safe() {
                continue;
            }
            let mut found = false;
            for planet in sp.get_planets() {
                let theme = planet.get_theme();
                if is_unknown && self.ocean_type == theme.water_item_id {
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
