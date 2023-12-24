use crate::data::planet::Planet;
use crate::data::rule::Rule;
use crate::data::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleThemeId {
    #[serde(skip)]
    pub evaluated: bool,
    pub theme_ids: Vec<i32>,
}

impl Rule for RuleThemeId {
    fn on_planets_themed(&mut self, _: &Star, planets: &Vec<Planet>) -> Option<bool> {
        self.evaluated = true;
        Some(
            self.theme_ids
                .iter()
                .all(|t| planets.iter().any(|p| *t == p.theme_proto.id)),
        )
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
