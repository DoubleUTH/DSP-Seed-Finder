#![cfg(target_arch = "wasm32")]

mod data;
mod rules;
mod transform_rules;
mod worldgen;

use data::game_desc::GameDesc;
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
pub fn generate(gameDesc: JsValue) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let game_desc: GameDesc = serde_wasm_bindgen::from_value(gameDesc)?;
    let galaxy = create_galaxy(&game_desc);
    serde_wasm_bindgen::to_value(&galaxy)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn findStars(gameDesc: JsValue, rule: JsValue) {
    spawn_local(async {
        let mut game_desc: GameDesc = serde_wasm_bindgen::from_value(gameDesc).unwrap();
        let rule = serde_wasm_bindgen::from_value(rule).unwrap();
        let mut transformed = transform_rules::transform_rules(rule);
        loop {
            let galaxy = find_stars(&game_desc, &mut transformed);
            let result = serde_wasm_bindgen::to_value(&galaxy).unwrap();
            let next_seed: JsValue = found(result).await;
            match next_seed.as_f64() {
                Some(f) => {
                    game_desc.seed = f as i32;
                }
                None => {
                    break;
                }
            }
        }
    })
}
