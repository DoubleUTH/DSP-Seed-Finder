mod worldgen;

use serde_json::to_string;
use wasm_bindgen::prelude::*;
use worldgen::{galaxy_gen::create_galaxy, game_desc::GameDesc};

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn generate(galaxySeed: i32, starCount: i32, resourceMultiplier: f32) -> String {
    let mut game_desc = GameDesc::new(galaxySeed);
    game_desc.star_count = starCount;
    game_desc.resource_multiplier = resourceMultiplier;
    let galaxy = create_galaxy(&game_desc);
    to_string(&galaxy).unwrap()
}
