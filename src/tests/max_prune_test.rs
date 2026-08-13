#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::data::enums::VeinType;
    use crate::data::game_desc::GameDesc;
    use crate::data::rule::Condition;
    use crate::rules::average_vein_amount::RuleAverageVeinAmount;
    use crate::transform_rules::{transform_rules, Rules};
    use crate::worldgen::galaxy_gen::{create_galaxy, find_stars};
    use rayon::prelude::*;

    const ALL_VEINS: [VeinType; 14] = [
        VeinType::Iron,
        VeinType::Copper,
        VeinType::Silicium,
        VeinType::Titanium,
        VeinType::Stone,
        VeinType::Coal,
        VeinType::Oil,
        VeinType::Fireice,
        VeinType::Diamond,
        VeinType::Fractal,
        VeinType::Crysrub,
        VeinType::Grat,
        VeinType::Bamboo,
        VeinType::Mag,
    ];

    fn game(resource_multiplier: f32) -> GameDesc {
        GameDesc {
            star_count: 64,
            resource_multiplier,
            hive_initial_colonize: 1.0,
            hive_max_density: 1.0,
            use_actual_veins: true,
        }
    }

    /// The soundness claim the prune rests on: for every star and vein type,
    /// the actual generated total never exceeds get_max_possible_vein.
    /// Exercised across resource multipliers because the bound's rounding
    /// slack term depends on the multiplier.
    #[test]
    fn actual_never_exceeds_bound() {
        for multiplier in [0.1_f32, 1.0, 8.0, 100.0] {
            let g = game(multiplier);
            let violations: Vec<String> = (0..12_i32)
                .into_par_iter()
                .flat_map(|seed| {
                    let habitable_count = Cell::new(0_i32);
                    let galaxy = create_galaxy(seed, &g, &habitable_count);
                    let mut bad = Vec::new();
                    for (index, sp) in galaxy.stars.iter().enumerate() {
                        for vein in ALL_VEINS {
                            let bound = sp.get_max_possible_vein(&vein);
                            let actual = sp.get_actual_vein(&vein) as f64;
                            if actual > bound {
                                bad.push(format!(
                                    "mult {} seed {} star {} {:?}: actual {} > bound {}",
                                    multiplier, seed, index, vein, actual, bound
                                ));
                            }
                        }
                    }
                    bad
                })
                .collect();
            assert!(violations.is_empty(), "{:#?}", violations);
        }
    }

    /// Rough single-thread timing of pruned actual-vein searches; run with
    ///   cargo test --release bench_prune -- --ignored --nocapture
    #[test]
    #[ignore]
    fn bench_prune() {
        let g = game(1.0);
        for (label, vein, threshold) in [
            ("iron>=45M", VeinType::Iron, 4.5e7_f32),
            ("iron>=60M", VeinType::Iron, 6.0e7),
            ("grat>=6M", VeinType::Grat, 6.0e6),
        ] {
            let rule = transform_rules(Rules::AverageVeinAmount(RuleAverageVeinAmount {
                use_actual: true,
                vein,
                condition: Condition::Gte(threshold),
            }));
            let n = 300;
            let start = std::time::Instant::now();
            let mut hits = 0;
            for seed in 0..n {
                if find_stars(seed, &g, &rule) != 0 {
                    hits += 1;
                }
            }
            let secs = start.elapsed().as_secs_f64();
            println!(
                "{}: {} seeds in {:.1}s = {:.1} seeds/s ({:.1} ms/seed), {} hits",
                label,
                n,
                secs,
                (n as f64) / secs,
                secs / (n as f64) * 1e3,
                hits
            );
        }

        // Realistic shape: a cheap star filter first, veins only on survivors.
        use crate::data::enums::SpectrType;
        use crate::rules::spectr::RuleSpectr;
        for (label, spectr, vein, threshold) in [
            ("O-star + iron>=40M", SpectrType::O, VeinType::Iron, 4.0e7_f32),
            ("M-star + grat>=3M", SpectrType::M, VeinType::Grat, 3.0e6),
        ] {
            let rule = transform_rules(Rules::And {
                rules: vec![
                    Rules::Spectr(RuleSpectr {
                        spectr: vec![spectr],
                    }),
                    Rules::AverageVeinAmount(RuleAverageVeinAmount {
                        use_actual: true,
                        vein,
                        condition: Condition::Gte(threshold),
                    }),
                ],
            });
            let n = 1000;
            let start = std::time::Instant::now();
            let mut hits = 0;
            for seed in 0..n {
                if find_stars(seed, &g, &rule) != 0 {
                    hits += 1;
                }
            }
            let secs = start.elapsed().as_secs_f64();
            println!(
                "{}: {} seeds in {:.1}s = {:.1} seeds/s ({:.1} ms/seed), {} hits",
                label,
                n,
                secs,
                (n as f64) / secs,
                secs / (n as f64) * 1e3,
                hits
            );
        }
    }

    /// The prune must not change rule results: compare find_stars (with the
    /// prune) against a direct per-star evaluation of the actual vein totals,
    /// across thresholds chosen to hit prune-reject, prune-accept, and
    /// indeterminate paths.
    #[test]
    fn pruned_rule_matches_direct_evaluation() {
        let conditions = [
            Condition::Gte(1e9),
            Condition::Gte(5_000_000.0),
            Condition::Gte(0.0),
            Condition::Gt(8_000_000.0),
            Condition::Lt(3_000_000.0),
            Condition::Lte(50_000_000.0),
            Condition::Eq(0.0),
            Condition::Neq(0.0),
            Condition::Between(2_000_000.0, 20_000_000.0),
            Condition::NotBetween(1.0, 1e9),
        ];
        let g = game(1.0);
        for vein in [VeinType::Iron, VeinType::Grat, VeinType::Oil, VeinType::Mag] {
            for condition in conditions.clone() {
                let rule = transform_rules(Rules::AverageVeinAmount(RuleAverageVeinAmount {
                    use_actual: true,
                    vein,
                    condition: condition.clone(),
                }));
                let mismatches: Vec<i32> = (0..25_i32)
                    .into_par_iter()
                    .filter(|&seed| {
                        let pruned = find_stars(seed, &g, &rule);
                        let habitable_count = Cell::new(0_i32);
                        let galaxy = create_galaxy(seed, &g, &habitable_count);
                        let mut direct = 0_u64;
                        for (index, sp) in galaxy.stars.iter().enumerate() {
                            if condition.eval(sp.get_actual_vein(&vein)) {
                                direct |= 1 << index;
                            }
                        }
                        pruned != direct
                    })
                    .collect();
                assert!(
                    mismatches.is_empty(),
                    "{:?} {:?}: prune changed results at seeds {:?}",
                    vein,
                    condition,
                    mismatches
                );
            }
        }
    }
}
