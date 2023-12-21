mod data;
mod rules;
mod worldgen;
mod transform_rules;

use data::{game_desc::GameDesc, rule::Rule, galaxy::Galaxy};
use wasm_bindgen::prelude::*;
use worldgen::galaxy_gen::{create_galaxy, find_stars};
use wasm_bindgen_futures::spawn_local;

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
        let mut transformed: Box<dyn Rule> = transform_rules::transform_rules(rule);
        loop {
            let stars = find_stars(&game_desc, &mut transformed);
            let galaxy = Galaxy { seed: game_desc.seed, stars };
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
