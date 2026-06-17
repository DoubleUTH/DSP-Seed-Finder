#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::data::game_desc::GameDesc;
    use crate::worldgen::galaxy_gen::create_galaxy;

    #[test]
    fn test_worldgen() {
        let game = GameDesc {
            star_count: 64,
            resource_multiplier: 1.0,
            hive_initial_colonize: 1.0,
            hive_max_density: 1.0,
            use_actual_veins: true,
        };
        let habitable_count = Cell::new(0_i32);
        let galaxy = create_galaxy(1, &game, &habitable_count);
        let _result = galaxy
            .stars
            .get(0)
            .unwrap()
            .get_planets()
            .get(3)
            .unwrap()
            .get_actual_veins();
    }
}
