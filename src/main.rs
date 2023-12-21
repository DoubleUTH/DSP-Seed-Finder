mod worldgen;

fn main() {
    println!("X");
    for _ in 0..10000 {
        let game = worldgen::game_desc::GameDesc::new(30502749);
        let _galaxy = worldgen::galaxy_gen::create_galaxy(&game);
        // println!("{0}", i);
    }
    println!("Y");
}
