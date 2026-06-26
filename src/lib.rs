#![cfg(target_arch = "wasm32")]

mod data;
mod rules;
mod transform_rules;
mod worldgen;

use data::game_desc::GameDesc;
use serde::Serialize;
use std::cell::Cell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use worldgen::galaxy_gen::{create_galaxy, find_stars};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = worldgen)]
    async fn found(value: JsValue) -> JsValue;
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn generate(seed: JsValue, gameDesc: JsValue) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let seed: i32 = serde_wasm_bindgen::from_value(seed)?;
    let game_desc: GameDesc = serde_wasm_bindgen::from_value(gameDesc)?;
    let habitable_count = Cell::new(0_i32);
    let galaxy = create_galaxy(seed, &game_desc, &habitable_count);
    galaxy.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn searchStar(
    seed: JsValue,
    gameDesc: JsValue,
    rule: JsValue,
) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let seed: i32 = serde_wasm_bindgen::from_value(seed)?;
    let game_desc: GameDesc = serde_wasm_bindgen::from_value(gameDesc)?;
    let rule = serde_wasm_bindgen::from_value(rule).unwrap();
    let transformed = transform_rules::transform_rules(rule);
    let star_indexes = find_stars(seed, &game_desc, &transformed);
    serde_wasm_bindgen::to_value(&star_indexes)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn findStars(gameDesc: JsValue, rule: JsValue, seeds: JsValue) {
    spawn_local(async {
        let game_desc: GameDesc = serde_wasm_bindgen::from_value(gameDesc).unwrap();
        let mut seeds: Vec<i32> = serde_wasm_bindgen::from_value(seeds).unwrap();
        let rule = serde_wasm_bindgen::from_value(rule).unwrap();
        let transformed = transform_rules::transform_rules(rule);
        loop {
            let mut results: Vec<i32> = vec![];
            for seed in seeds {
                let star_indexes = find_stars(seed, &game_desc, &transformed);
                if !star_indexes.is_empty() {
                    results.push(seed);
                }
            }
            let result = serde_wasm_bindgen::to_value(&results).unwrap();
            let next_batch: JsValue = found(result).await;
            let next_seeds: Result<Vec<i32>, serde_wasm_bindgen::Error> =
                serde_wasm_bindgen::from_value(next_batch);
            match next_seeds {
                Ok(f) => {
                    seeds = f;
                }
                Err(_) => {
                    break;
                }
            }
        }
    })
}
