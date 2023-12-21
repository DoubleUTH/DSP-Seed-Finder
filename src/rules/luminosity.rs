use crate::data::rule::{Condition, Rule};
use crate::data::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RuleLuminosity {
    #[serde(skip)]
    pub evaluated: bool,
    pub condition: Condition,
}

#[typetag::serde]
impl Rule for RuleLuminosity {
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn on_star_created(&mut self, star: &Star) -> Option<bool> {
        self.evaluated = true;
        self.condition.eval(star.luminosity)
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
