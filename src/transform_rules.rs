use crate::rules;
use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Rules {
    And { rules: Vec<Rules> },
    Or { rules: Vec<Rules> },
    Luminosity(rules::luminosity::RuleLuminosity),
    DysonRadius(rules::dyson_radius::RuleDysonRadius),
}

pub fn transform_rules(r: Rules) -> Box<dyn Rule> {
    match r {
        Rules::Luminosity(rule) => Box::new(rule),
        Rules::DysonRadius(rule) => Box::new(rule),
        Rules::And { rules } => Box::new(rules::and::RuleAnd { evaluated: false, rules: rules.into_iter().map(transform_rules).collect() }),
        Rules::Or { rules } => Box::new(rules::or::RuleOr { evaluated: false, rules: rules.into_iter().map(transform_rules).collect() }),
    }
}
