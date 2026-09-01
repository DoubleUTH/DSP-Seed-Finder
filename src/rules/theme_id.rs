use crate::data::galaxy::Galaxy;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::evaluate_unsafe;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleThemeId {
    pub theme_ids: Vec<i32>,
    pub negate: bool,
}

impl Rule for RuleThemeId {
    fn get_priority(&self) -> i32 {
        40
    }

    fn need_position(&self) -> bool {
        false
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        evaluate_unsafe!(galaxy, evaluation, |sp| {
            let mut found = false;
            for planet in sp.get_planets() {
                let theme = planet.get_theme();
                if self.theme_ids.contains(&theme.id) {
                    found = true;
                }
            }
            found != self.negate
        })
    }
}
