use crate::data::rule::Rule;
use crate::data::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleBirth {
    #[serde(skip)]
    pub evaluated: bool,
}

impl Rule for RuleBirth {
    fn on_star_created(&mut self, star: &Star) -> Option<bool> {
        self.evaluated = true;
        Some(star.id == 1)
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
