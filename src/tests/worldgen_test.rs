#[cfg(test)]
mod tests {
    use crate::data::game_desc::GameDesc;
    use crate::worldgen::galaxy_gen::create_galaxy;

    #[test]
    fn test_worldgen() {
        let game = GameDesc {
            seed: 1,
            star_count: 64,
            resource_multiplier: 1.0,
            habitable_count: Default::default(),
            hive_initial_colonize: 1.0,
            hive_max_density: 1.0,
            use_actual_veins: true,
        };
        let galaxy = create_galaxy(&game);
        let _result = galaxy
            .stars
            .get(1)
            .unwrap()
            .get_planets()
            .get(1)
            .unwrap()
            .get_actual_veins();
    }
}
