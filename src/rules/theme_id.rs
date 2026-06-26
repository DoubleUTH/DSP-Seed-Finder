use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleThemeId {
    pub theme_ids: Vec<i32>,
}

impl Rule for RuleThemeId {
    fn get_priority(&self) -> i32 {
        40
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> u64 {
        let mut result: u64 = 0;
        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            let planets = sp.get_planets();
            if evaluation.is_known(index) {
                if !sp.is_safe() {
                    sp.load_planets()
                }
                continue;
            }
            let mut found = false;
            for planet in planets {
                let theme = planet.get_theme();
                if self.theme_ids.contains(&theme.id) {
                    found = true;
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
