use crate::data::enums::VeinType;
use crate::data::rule::{Condition, Rule};
use crate::data::planet::Planet;
use crate::data::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleAverageVeinPatch {
    #[serde(skip)]
    pub evaluated: bool,
    pub vein: VeinType,
    pub condition: Condition,
}

impl Rule for RuleAverageVeinPatch {
    fn on_veins_generated(&mut self, star: &Star, _: &Vec<Planet>) -> Option<bool> {
        self.evaluated = true;
        let value = if let Some(x) = star.vein_patch.get(&self.vein) {
            *x
        } else {
            0.0
        };
        Some(self.condition.eval(value))
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
