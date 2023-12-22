use crate::data::rule::{Condition, Rule};
use crate::data::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleDysonRadius {
    #[serde(skip)]
    pub evaluated: bool,
    pub condition: Condition,
}

impl Rule for RuleDysonRadius {
    fn on_star_created(&mut self, star: &Star) -> Option<bool> {
        self.evaluated = true;
        Some(self.condition.eval(star.dyson_radius))
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
