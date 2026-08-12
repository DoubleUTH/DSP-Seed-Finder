#[cfg(test)]
mod tests {
    use crate::data::enums::{SpectrType, StarType};
    use crate::data::game_desc::GameDesc;
    use crate::data::rule::Condition;
    use crate::rules;
    use crate::transform_rules::{transform_rules, Rules};
    use crate::worldgen::galaxy_gen::{find_stars_full, find_stars_positionless};
    use rayon::prelude::*;

    // Equivalence proof for the positionless fast path: for every rule that
    // declares needs_walk() == false, the bitmask from the positionless galaxy
    // must equal the bitmask from the fully generated galaxy — including
    // rejections, since a mismatch there would silently drop matching seeds.
    // If a future change makes an allowlisted rule read anything derived from
    // star positions, this test fails loudly.

    const SEEDS: i32 = 2000;

    fn game(star_count: usize) -> GameDesc {
        GameDesc {
            star_count,
            resource_multiplier: 1.0,
            hive_initial_colonize: 1.0,
            hive_max_density: 1.0,
            use_actual_veins: false,
        }
    }

    fn position_free_battery() -> Vec<(&'static str, Rules)> {
        vec![
            ("birth", Rules::Birth(rules::birth::RuleBirth {})),
            (
                "luminosity",
                Rules::Luminosity(rules::luminosity::RuleLuminosity {
                    condition: Condition::Gte(2.0),
                }),
            ),
            (
                "star_type",
                Rules::StarType(rules::star_type::RuleStarType {
                    star_type: vec![
                        StarType::BlackHole,
                        StarType::NeutronStar,
                        StarType::GiantStar,
                    ],
                }),
            ),
            (
                "spectr",
                Rules::Spectr(rules::spectr::RuleSpectr {
                    spectr: vec![SpectrType::O, SpectrType::M],
                }),
            ),
            (
                "dyson_radius",
                Rules::DysonRadius(rules::dyson_radius::RuleDysonRadius {
                    condition: Condition::Gte(30000.0),
                }),
            ),
            (
                "planet_count",
                Rules::PlanetCount(rules::planet_count::RulePlanetCount {
                    exclude_giant: false,
                    condition: Condition::Gte(5.0),
                }),
            ),
            (
                "satellite_count",
                Rules::SatelliteCount(rules::satellite_count::RuleSatelliteCount {
                    condition: Condition::Gte(1.0),
                }),
            ),
            (
                "tidal_lock_count",
                Rules::TidalLockCount(rules::tidal_lock_count::RuleTidalLockCount {
                    condition: Condition::Gte(1.0),
                }),
            ),
            (
                "theme_id",
                Rules::ThemeId(rules::theme_id::RuleThemeId {
                    theme_ids: vec![1, 6, 7, 12, 13],
                }),
            ),
            (
                "ocean_type",
                Rules::OceanType(rules::ocean_type::RuleOceanType { ocean_type: 1000 }),
            ),
            (
                "gas_count",
                Rules::GasCount(rules::gas_count::RuleGasCount {
                    ice: None,
                    condition: Condition::Gte(2.0),
                }),
            ),
            (
                "gas_count_ice",
                Rules::GasCount(rules::gas_count::RuleGasCount {
                    ice: Some(true),
                    condition: Condition::Gte(1.0),
                }),
            ),
            (
                "planet_in_dyson_count",
                Rules::PlanetInDysonCount(
                    rules::planet_in_dyson_count::RulePlanetInDysonCount {
                        include_giant: false,
                        condition: Condition::Gte(1.0),
                    },
                ),
            ),
            (
                "hive_count_max",
                Rules::HiveCount(rules::hive_count::RuleHiveCount {
                    condition: Condition::Gte(2.0),
                    initial: false,
                }),
            ),
            (
                "and",
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
            ),
            (
                "or",
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
            ),
            (
                "composite",
                Rules::Composite {
                    rule: Box::new(Rules::Spectr(rules::spectr::RuleSpectr {
                        spectr: vec![SpectrType::M],
                    })),
                    condition: Condition::Gte(8.0),
                },
            ),
            (
                "composite_and",
                Rules::CompositeAnd {
                    rules: vec![
                        Rules::Composite {
                            rule: Box::new(Rules::StarType(rules::star_type::RuleStarType {
                                star_type: vec![StarType::BlackHole],
                            })),
                            condition: Condition::Gte(1.0),
                        },
                        Rules::Composite {
                            rule: Box::new(Rules::PlanetCount(
                                rules::planet_count::RulePlanetCount {
                                    exclude_giant: false,
                                    condition: Condition::Gte(6.0),
                                },
                            )),
                            condition: Condition::Gte(1.0),
                        },
                    ],
                },
            ),
        ]
    }

    #[test]
    fn position_free_rules_declare_no_walk() {
        for (name, rule) in position_free_battery() {
            assert!(
                !transform_rules(rule).needs_walk(),
                "{} unexpectedly needs the walk",
                name
            );
        }
    }

    #[test]
    fn position_dependent_rules_declare_walk() {
        use crate::data::enums::VeinType;
        let dependent: Vec<(&str, Rules)> = vec![
            (
                "birth_distance",
                Rules::BirthDistance(rules::birth_distance::RuleBirthDistance {
                    condition: Condition::Lte(20.0),
                }),
            ),
            (
                "x_distance",
                Rules::XDistance(rules::x_distance::RuleXDistance {
                    condition: Condition::Lte(15.0),
                    all: false,
                }),
            ),
            (
                "spectr_distance",
                Rules::SpectrDistance(rules::spectr_distance::RuleSpectrDistance {
                    spectr: SpectrType::O,
                    distance_condition: Condition::Lte(30.0),
                    count_condition: Condition::Gte(1.0),
                }),
            ),
            (
                "vein_estimated",
                Rules::AverageVeinAmount(rules::average_vein_amount::RuleAverageVeinAmount {
                    use_actual: false,
                    vein: VeinType::Iron,
                    condition: Condition::Gte(1.0),
                }),
            ),
            (
                "vein_actual",
                Rules::AverageVeinAmount(rules::average_vein_amount::RuleAverageVeinAmount {
                    use_actual: true,
                    vein: VeinType::Iron,
                    condition: Condition::Gte(1.0),
                }),
            ),
            (
                "gas_rate",
                Rules::GasRate(rules::gas_rate::RuleGasRate {
                    gas_type: 1120,
                    condition: Condition::Gte(0.5),
                }),
            ),
            (
                "hive_count_initial",
                Rules::HiveCount(rules::hive_count::RuleHiveCount {
                    condition: Condition::Gte(1.0),
                    initial: true,
                }),
            ),
            (
                "and_tainted",
                Rules::And {
                    rules: vec![
                        Rules::Luminosity(rules::luminosity::RuleLuminosity {
                            condition: Condition::Gte(1.0),
                        }),
                        Rules::BirthDistance(rules::birth_distance::RuleBirthDistance {
                            condition: Condition::Lte(20.0),
                        }),
                    ],
                },
            ),
        ];
        for (name, rule) in dependent {
            assert!(
                transform_rules(rule).needs_walk(),
                "{} must declare needs_walk",
                name
            );
        }
    }

    #[test]
    fn positionless_equals_full_generation() {
        for star_count in [32, 48, 64] {
            let g = game(star_count);
            for (name, rule) in position_free_battery() {
                let rule = transform_rules(rule);
                let mismatches: Vec<i32> = (0..SEEDS)
                    .into_par_iter()
                    .filter(|&seed| {
                        find_stars_positionless(seed, &g, &rule)
                            != find_stars_full(seed, &g, &rule)
                    })
                    .collect();
                assert!(
                    mismatches.is_empty(),
                    "rule {} star_count {}: positionless bitmask diverges at seeds {:?}",
                    name,
                    star_count,
                    &mismatches[..mismatches.len().min(10)]
                );
            }
        }
    }
}
