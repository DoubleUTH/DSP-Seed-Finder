use crate::rules;
use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Rules {
    And { rules: Vec<Rules> },
    Or { rules: Vec<Rules> },
    Luminosity(rules::luminosity::RuleLuminosity),
    DysonRadius(rules::dyson_radius::RuleDysonRadius),
    AverageVeinAmount(rules::average_vein_amount::RuleAverageVeinAmount),
    AverageVeinPatch(rules::average_vein_patch::RuleAverageVeinPatch),
    Spectr(rules::spectr::RuleSpectr),
    TidalLockCount(rules::tidal_lock_count::RuleTidalLockCount),
    OceanType(rules::ocean_type::RuleOceanType),
    StarType(rules::star_type::RuleStarType),
    GasCount(rules::gas_count::RuleGasCount),
    SatelliteCount(rules::satellite_count::RuleSatelliteCount),
    Birth(rules::birth::RuleBirth),
}

pub fn transform_rules(r: Rules) -> Box<dyn Rule + Send> {
    match r {
        Rules::And { rules } => Box::new(rules::and::RuleAnd { evaluated: false, rules: rules.into_iter().map(transform_rules).collect() }),
        Rules::Or { rules } => Box::new(rules::or::RuleOr { evaluated: false, rules: rules.into_iter().map(transform_rules).collect() }),
        Rules::Luminosity(rule) => Box::new(rule),
        Rules::DysonRadius(rule) => Box::new(rule),
        Rules::AverageVeinAmount(rule) => Box::new(rule),
        Rules::AverageVeinPatch(rule) => Box::new(rule),
        Rules::Spectr(rule) => Box::new(rule),
        Rules::TidalLockCount(rule) => Box::new(rule),
        Rules::OceanType(rule) => Box::new(rule),
        Rules::StarType(rule) => Box::new(rule),
        Rules::GasCount(rule) => Box::new(rule),
        Rules::SatelliteCount(rule) => Box::new(rule),
        Rules::Birth(rule) => Box::new(rule),
    }
}
