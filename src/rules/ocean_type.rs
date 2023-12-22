use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

// TODO: add ocean type to theme proto

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleOceanType {
    #[serde(skip)]
    pub evaluated: bool,
    pub ocean_type: Vec<i32>,
}

impl Rule for RuleOceanType {
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
