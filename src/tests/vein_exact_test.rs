#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::data::enums::VeinType;
    use crate::data::game_desc::GameDesc;
    use crate::worldgen::galaxy_gen::create_galaxy;

    const SEEDS: i32 = 40;

    fn game() -> GameDesc {
        GameDesc {
            star_count: 64,
            resource_multiplier: 1.0,
            hive_initial_colonize: 1.0,
            hive_max_density: 1.0,
            use_actual_veins: true,
        }
    }

    // Mag is special-cased (returns 0 unless the star is a black hole or
    // neutron star), so it is excluded from the ground-truth comparison.
    const VEINS: &[VeinType] = &[
        VeinType::Iron,
        VeinType::Copper,
        VeinType::Silicium,
        VeinType::Titanium,
        VeinType::Stone,
        VeinType::Coal,
    ];

    /// `get_actual_vein_exact` must equal the plain sum of the per-planet
    /// amounts -- that sum is the definition of the quantity.
    #[test]
    fn exact_equals_planet_sum() {
        let g = game();
        for seed in 0..SEEDS {
            let habitable_count = Cell::new(0_i32);
            let galaxy = create_galaxy(seed, &g, &habitable_count);
            for sp in galaxy.stars.iter() {
                for vein in VEINS {
                    let expected: i64 = sp
                        .get_planets()
                        .iter()
                        .flat_map(|p| p.get_actual_veins().iter())
                        .filter(|v| &v.vein_type == vein)
                        .map(|v| v.amount as i64)
                        .sum();
                    assert_eq!(
                        sp.get_actual_vein_exact(vein),
                        expected,
                        "seed {seed} star {} vein {vein:?}",
                        sp.star.index
                    );
                }
            }
        }
    }

    /// Documents WHY the exact accessor exists: an `f32` cannot represent every
    /// integer above 2^24, and real star totals are far larger than that, so the
    /// `f32` view is lossy for a substantial share of stars.
    #[test]
    fn f32_view_is_lossy_above_2p24() {
        let g = game();
        let mut large = 0_u32;
        let mut lossy = 0_u32;
        for seed in 0..SEEDS {
            let habitable_count = Cell::new(0_i32);
            let galaxy = create_galaxy(seed, &g, &habitable_count);
            for sp in galaxy.stars.iter() {
                for vein in VEINS {
                    let exact = sp.get_actual_vein_exact(vein);
                    if exact > (1 << 24) {
                        large += 1;
                        if (sp.get_actual_vein(vein) as i64) != exact {
                            lossy += 1;
                        }
                    }
                }
            }
        }
        assert!(large > 0, "expected some totals above 2^24 in {SEEDS} seeds");
        assert!(
            lossy > 0,
            "expected the f32 view to lose precision on at least one total \
             above 2^24 ({large} such totals seen)"
        );
        // Never the other way round: below 2^24 an f32 is exact.
        println!("{lossy}/{large} totals above 2^24 are rounded by the f32 view");
    }
}
