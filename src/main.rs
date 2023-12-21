mod worldgen;

fn main() {
    let game = worldgen::game_desc::GameDesc::new(30502749);
    let galaxy = worldgen::galaxy_gen::create_galaxy(&game);
    println!("{:?}", galaxy);
}
