use dsp_seed_finder::data::game_desc::GameDesc;
use dsp_seed_finder::worldgen::galaxy_gen::create_galaxy;
use serde_json::to_string;

fn main() {
    for seed in 0..10 {
        let game_desc = GameDesc {
            seed,
            star_count: 64, // Assuming a default of 64 stars, as seen in the frontend
            resource_multiplier: 1.0,
        };
        let galaxy = create_galaxy(&game_desc);
        let galaxy_json = to_string(&galaxy).unwrap();
        println!("{}", galaxy_json);
    }
}
