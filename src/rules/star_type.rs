use crate::data::rule::Rule;
use crate::data::enums::StarType;
use crate::data::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleStarType {
    #[serde(skip)]
    pub evaluated: bool,
    pub star_type: Vec<StarType>,
}

impl Rule for RuleStarType {
    fn on_star_created(&mut self, star: &Star) -> Option<bool> {
        self.evaluated = true;
        Some(self.star_type.contains(&star.star_type))
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
