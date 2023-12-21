mod data;
mod rules;
mod worldgen;

use data::{game_desc::GameDesc, rule::Rule};
use rules::{and::RuleAnd, dyson_radius::RuleDysonRadius, luminosity::RuleLuminosity, or::RuleOr};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use worldgen::galaxy_gen::{create_galaxy, find_stars};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Rules {
    And { rules: Vec<Rules> },
    Or { rules: Vec<Rules> },
    Luminosity(RuleLuminosity),
    DysonRadius(RuleDysonRadius),
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn generate(gameDesc: JsValue) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let game_desc: GameDesc = serde_wasm_bindgen::from_value(gameDesc)?;
    let galaxy = create_galaxy(&game_desc);
    serde_wasm_bindgen::to_value(&galaxy)
}

fn transform_rules(r: Rules) -> Box<dyn Rule> {
    match r {
        Rules::Luminosity(rule) => Box::new(rule),
        Rules::DysonRadius(rule) => Box::new(rule),
        Rules::And { rules } => Box::new(RuleAnd { evaluated: false, rules: rules.into_iter().map(transform_rules).collect() }),
        Rules::Or { rules } => Box::new(RuleOr { evaluated: false, rules: rules.into_iter().map(transform_rules).collect() }),
    }
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn findStars(gameDesc: JsValue, rule: JsValue) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let game_desc: GameDesc = serde_wasm_bindgen::from_value(gameDesc)?;
    let rule = serde_wasm_bindgen::from_value(rule)?;
    let transformed = transform_rules(rule);
    let stars = find_stars(&game_desc, transformed);
    serde_wasm_bindgen::to_value(&stars)
}

#[cfg(test)]
mod tests {
    use crate::{Rules, data::rule::Condition};

    #[test]
    fn rule_deserialize() {
        let str = r#"{
            "type": "And",
            "rules": [
                {
                    "type": "Luminosity",
                    "condition": {
                        "type": "Between",
                        "value": [0.0, 1.0]
                    }
                }
            ]
        }"#;
        let result: Rules = serde_json::from_str(str).unwrap();
        match result {
            Rules::And { rules } => {
                assert_eq!(rules.len(), 1);
                match &rules[0] {
                    Rules::Luminosity(r) => {
                        assert_eq!(r.condition, Condition::Between(0.0, 1.0))
                    },
                    _ => panic!("not Luminosity type"),
                }
            },
            _ => panic!("not And type")
        }
    }
}

