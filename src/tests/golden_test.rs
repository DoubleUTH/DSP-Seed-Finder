#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::data::enums::{SpectrType, StarType, VeinType};
    use crate::data::game_desc::GameDesc;
    use crate::data::rule::Condition;
    use crate::rules;
    use crate::transform_rules::{transform_rules, Rules};
    use crate::worldgen::galaxy_gen::{create_galaxy, find_stars};
    use rayon::prelude::*;

    // Golden regression tests for the generation + rule-evaluation invariant:
    // any change to worldgen or rule code must reproduce these values exactly,
    // because reported seeds must match what the game generates.
    //
    // Run with `cargo test --release` (debug mode works but is ~10x slower).
    // After an INTENTIONAL behavior change, regenerate the constants with:
    //   cargo test --release print_goldens -- --ignored --nocapture

    const CHEAP_SEEDS: i32 = 600;
    const VEIN_EST_SEEDS: i32 = 100;

    fn game(star_count: usize) -> GameDesc {
        GameDesc {
            star_count,
            resource_multiplier: 1.0,
            hive_initial_colonize: 1.0,
            hive_max_density: 1.0,
            use_actual_veins: false,
        }
    }

    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    fn fnv1a_u64(mut hash: u64, value: u64) -> u64 {
        for byte in value.to_le_bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        hash
    }

    /// One rule per rule type (plus composite forms), with conditions loose
    /// enough to produce non-trivial bitmasks on ordinary seeds.
    fn cheap_battery() -> Vec<Rules> {
        vec![
            Rules::Birth(rules::birth::RuleBirth {}),
            Rules::Luminosity(rules::luminosity::RuleLuminosity {
                condition: Condition::Gte(2.0),
            }),
            Rules::StarType(rules::star_type::RuleStarType {
                star_type: vec![StarType::BlackHole, StarType::NeutronStar],
            }),
            Rules::Spectr(rules::spectr::RuleSpectr {
                spectr: vec![SpectrType::O, SpectrType::M],
            }),
            Rules::DysonRadius(rules::dyson_radius::RuleDysonRadius {
                condition: Condition::Gte(30000.0),
            }),
            Rules::PlanetCount(rules::planet_count::RulePlanetCount {
                exclude_giant: false,
                condition: Condition::Gte(5.0),
            }),
            Rules::PlanetCount(rules::planet_count::RulePlanetCount {
                exclude_giant: true,
                condition: Condition::Gte(4.0),
            }),
            Rules::SatelliteCount(rules::satellite_count::RuleSatelliteCount {
                condition: Condition::Gte(1.0),
            }),
            Rules::TidalLockCount(rules::tidal_lock_count::RuleTidalLockCount {
                condition: Condition::Gte(1.0),
            }),
            Rules::ThemeId(rules::theme_id::RuleThemeId {
                theme_ids: vec![1, 6, 7, 12, 13],
                negate: false,
            }),
            Rules::OceanType(rules::ocean_type::RuleOceanType { ocean_type: 1000 }),
            Rules::GasCount(rules::gas_count::RuleGasCount {
                ice: None,
                condition: Condition::Gte(2.0),
            }),
            Rules::GasCount(rules::gas_count::RuleGasCount {
                ice: Some(true),
                condition: Condition::Gte(1.0),
            }),
            Rules::GasRate(rules::gas_rate::RuleGasRate {
                gas_type: 1120,
                condition: Condition::Gte(0.5),
            }),
            Rules::PlanetInDysonCount(rules::planet_in_dyson_count::RulePlanetInDysonCount {
                include_giant: false,
                condition: Condition::Gte(1.0),
            }),
            Rules::HiveCount(rules::hive_count::RuleHiveCount {
                condition: Condition::Gte(2.0),
                initial: false,
            }),
            Rules::HiveCount(rules::hive_count::RuleHiveCount {
                condition: Condition::Gte(1.0),
                initial: true,
            }),
            Rules::BirthDistance(rules::birth_distance::RuleBirthDistance {
                condition: Condition::Lte(20.0),
            }),
            Rules::XDistance(rules::x_distance::RuleXDistance {
                condition: Condition::Lte(15.0),
                all: false,
            }),
            Rules::SpectrDistance(rules::spectr_distance::RuleSpectrDistance {
                spectr: SpectrType::O,
                distance_condition: Condition::Lte(30.0),
                count_condition: Condition::Gte(1.0),
            }),
            Rules::And {
                rules: vec![
                    Rules::Luminosity(rules::luminosity::RuleLuminosity {
                        condition: Condition::Gte(1.0),
                    }),
                    Rules::PlanetCount(rules::planet_count::RulePlanetCount {
                        exclude_giant: false,
                        condition: Condition::Gte(4.0),
                    }),
                ],
            },
            Rules::Or {
                rules: vec![
                    Rules::StarType(rules::star_type::RuleStarType {
                        star_type: vec![StarType::GiantStar],
                    }),
                    Rules::TidalLockCount(rules::tidal_lock_count::RuleTidalLockCount {
                        condition: Condition::Gte(2.0),
                    }),
                ],
            },
            Rules::Composite {
                rule: Box::new(Rules::Spectr(rules::spectr::RuleSpectr {
                    spectr: vec![SpectrType::M],
                })),
                condition: Condition::Gte(8.0),
            },
            Rules::CompositeAnd {
                rules: vec![
                    Rules::Composite {
                        rule: Box::new(Rules::StarType(rules::star_type::RuleStarType {
                            star_type: vec![StarType::BlackHole],
                        })),
                        condition: Condition::Gte(1.0),
                    },
                    Rules::Composite {
                        rule: Box::new(Rules::PlanetCount(rules::planet_count::RulePlanetCount {
                            exclude_giant: false,
                            condition: Condition::Gte(6.0),
                        })),
                        condition: Condition::Gte(1.0),
                    },
                ],
            },
        ]
    }

    fn vein_battery() -> Vec<Rules> {
        vec![
            Rules::AverageVeinAmount(rules::average_vein_amount::RuleAverageVeinAmount {
                use_actual: false,
                vein: VeinType::Iron,
                condition: Condition::Gte(8_000_000.0),
            }),
            Rules::AverageVeinAmount(rules::average_vein_amount::RuleAverageVeinAmount {
                use_actual: false,
                vein: VeinType::Grat,
                condition: Condition::Gte(1.0),
            }),
        ]
    }

    fn battery_fingerprint(battery: Vec<Rules>, star_count: usize, seeds: i32) -> u64 {
        let g = game(star_count);
        let mut hash = FNV_OFFSET;
        for rule in battery {
            let rule = transform_rules(rule);
            let masks: Vec<u64> = (0..seeds)
                .into_par_iter()
                .map(|seed| find_stars(seed, &g, &rule))
                .collect();
            for mask in masks {
                hash = fnv1a_u64(hash, mask);
            }
        }
        hash
    }

    fn cheap_fingerprint(star_count: usize) -> u64 {
        battery_fingerprint(cheap_battery(), star_count, CHEAP_SEEDS)
    }

    fn vein_est_fingerprint(star_count: usize) -> u64 {
        battery_fingerprint(vein_battery(), star_count, VEIN_EST_SEEDS)
    }

    /// Galaxy-wide actual-vein totals for a handful of seeds; also covers the
    /// full planet/theme/terrain pipeline. f64 sum of per-star f32 totals is
    /// deterministic; stored as bits.
    fn actual_vein_totals(seed: i32) -> [u64; 4] {
        let g = GameDesc {
            use_actual_veins: true,
            ..game(64)
        };
        let habitable_count = Cell::new(0_i32);
        let galaxy = create_galaxy(seed, &g, &habitable_count);
        [
            VeinType::Iron,
            VeinType::Copper,
            VeinType::Oil,
            VeinType::Mag,
        ]
        .map(|vein| {
            let total: f64 = galaxy
                .stars
                .iter()
                .map(|sp| sp.get_actual_vein(&vein) as f64)
                .sum();
            total.to_bits()
        })
    }

    // ==================== GOLDEN VALUES ====================
    // Generated from upstream main @ 06cb3d8 (trusted reference).

    const CHEAP_GOLDENS: &[(usize, u64)] = &[
        (32, 0x1bcb82012e96a4e0),
        (48, 0x2de8ef044b998696),
        (64, 0x84836fe563b0edc3),
    ];

    const VEIN_EST_GOLDENS: &[(usize, u64)] = &[(32, 0x06862c519e804c99), (64, 0xd4cec694faa6fe9a)];

    const ACTUAL_VEIN_GOLDENS: &[(i32, [u64; 4])] = &[
        (
            1,
            [
                0x41f00447f6b00000,
                0x41ef8ce1c5600000,
                0x41869ea888000000,
                0x415668abc0000000,
            ],
        ),
        (
            42,
            [
                0x41f630ecc1d00000,
                0x41f79fca2e900000,
                0x4187a414e8000000,
                0x4151737080000000,
            ],
        ),
        (
            98765432,
            [
                0x41f6644d03f00000,
                0x41f79959e2800000,
                0x418ad28f48000000,
                0x414f557f80000000,
            ],
        ),
    ];

    #[test]
    fn golden_cheap_bitmasks() {
        for (star_count, expected) in CHEAP_GOLDENS {
            assert_eq!(
                cheap_fingerprint(*star_count),
                *expected,
                "cheap-rule bitmask fingerprint changed at star_count={}",
                star_count
            );
        }
    }

    #[test]
    fn golden_estimated_vein_bitmasks() {
        for (star_count, expected) in VEIN_EST_GOLDENS {
            assert_eq!(
                vein_est_fingerprint(*star_count),
                *expected,
                "estimated-vein bitmask fingerprint changed at star_count={}",
                star_count
            );
        }
    }

    #[test]
    fn golden_actual_vein_totals() {
        for (seed, expected) in ACTUAL_VEIN_GOLDENS {
            assert_eq!(
                &actual_vein_totals(*seed),
                expected,
                "actual-vein totals changed at seed={}",
                seed
            );
        }
    }

    #[test]
    #[ignore]
    fn print_goldens() {
        for star_count in [32, 48, 64] {
            println!(
                "cheap {}: 0x{:016x}",
                star_count,
                cheap_fingerprint(star_count)
            );
        }
        for star_count in [32, 64] {
            println!(
                "vein_est {}: 0x{:016x}",
                star_count,
                vein_est_fingerprint(star_count)
            );
        }
        for seed in [1, 42, 98765432] {
            let totals = actual_vein_totals(seed);
            println!(
                "actual {}: [0x{:016x}, 0x{:016x}, 0x{:016x}, 0x{:016x}]",
                seed, totals[0], totals[1], totals[2], totals[3]
            );
        }
    }
}
