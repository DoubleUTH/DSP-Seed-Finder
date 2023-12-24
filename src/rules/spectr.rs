use crate::data::enums::SpectrType;
use crate::data::planet::Planet;
use crate::data::rule::Rule;
use crate::data::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleSpectr {
    #[serde(skip)]
    pub evaluated: bool,
    pub spectr: Vec<SpectrType>,
}

impl Rule for RuleSpectr {
    fn on_planets_created(&mut self, star: &Star, _: &Vec<Planet>) -> Option<bool> {
        self.evaluated = true;
        Some(self.spectr.contains(&star.spectr))
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
