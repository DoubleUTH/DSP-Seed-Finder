use crate::data::planet::Planet;
use crate::data::rule::Rule;
use crate::data::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleBirth {
    #[serde(skip)]
    pub evaluated: bool,
}

impl Rule for RuleBirth {
    fn on_planets_created(&mut self, star: &Star, _: &Vec<Planet>) -> Option<bool> {
        self.evaluated = true;
        Some(star.index == 0)
    }
    fn is_birth(&self) -> bool {
        true
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
