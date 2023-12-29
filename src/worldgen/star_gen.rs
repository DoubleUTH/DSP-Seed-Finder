use super::planet_gen::create_planet;
use super::random::DspRandom;
use crate::data::enums::{SpectrType, StarType};
use crate::data::planet::Planet;
use crate::data::star::Star;
use crate::data::vector3::Vector3;

pub fn create_star(
    star_count: i32,
    pos: &Vector3,
    id: i32,
    seed: i32,
    need_type: StarType,
    need_spectr: SpectrType,
) -> Star {
    let mut star = Star::new();
    star.index = id - 1;
    star.level = (star.index as f32) / ((star_count - 1) as f32);
    star.id = id;
    star.seed = seed;
    let mut rand1 = DspRandom::new(seed);
    star.name_seed = rand1.next_seed();
    star.position = pos.clone();
    let magitude = pos.magnitude() as f32;
    let mut num1 = magitude / 32.0;
    if (num1 as f64) > 1.0 {
        num1 = ((((num1.ln() + 1.0).ln() + 1.0).ln() + 1.0).ln() + 1.0).ln() + 1.0
    }
    star.resource_coef = 7.0_f32.powf(num1) * 0.6;
    let mut rand2 = DspRandom::new(rand1.next_seed());
    let r1_1 = rand2.next_f64();
    let r2_1 = rand2.next_f64();
    let num2 = rand2.next_f64();
    let rn = rand2.next_f64();
    let rt = rand2.next_f64();
    let num3 = (rand2.next_f64() - 0.5) * 0.2;
    let num4 = rand2.next_f64() * 0.2 + 0.9;
    let y = rand2.next_f64() * 0.4 - 0.2;
    let num5 = 2_f64.powf(y);
    let num7 = -0.98 + (0.88 + 0.98) * star.level.clamp(0.0, 1.0);
    let average_value = if need_type == StarType::GiantStar {
        if y > -0.08 {
            -1.5
        } else {
            1.6
        }
    } else if num7 >= 0.0 {
        num7 + 0.65
    } else {
        num7 - 0.65
    };
    let standard_deviation = if need_type == StarType::GiantStar {
        0.3_f32
    } else {
        0.33_f32
    };

    let num8 = match need_spectr {
        SpectrType::M => -3_f32,
        SpectrType::O => 3_f32,
        _ => rand_normal(average_value, standard_deviation, r1_1, r2_1),
    };
    let p1 = ((if num8 <= 0.0 { num8 } else { num8 * 2.0 }).clamp(-2.4, 4.65) as f64 + num3 + 1.0)
        as f32;

    star.mass = match need_type {
        StarType::WhiteDwarf => (1.0 + r2_1 * 5.0) as f32,
        StarType::NeutronStar => (7.0 + r1_1 * 11.0) as f32,
        StarType::BlackHole => (18.0 + r1_1 * r2_1 * 30.0) as f32,
        _ => 2_f32.powf(p1),
    };

    let d = if star.mass < 2.0 {
        2.0 + 0.4 * (1.0 - (star.mass as f64))
    } else {
        5.0
    };

    star.lifetime = (10000.0
        * 0.1_f64.powf(((star.mass as f64) * 0.5).log10() / d.log10() + 1.0)
        * num4) as f32;

    match need_type {
        StarType::GiantStar => {
            star.lifetime = (10000.0
                * 0.1_f64.powf(((star.mass as f64) * 0.58).log10() / d.log10() + 1.0)
                * num4) as f32;
            star.age = (num2 * 0.0399999991059303 + 0.959999978542328) as f32;
        }
        StarType::WhiteDwarf | StarType::NeutronStar | StarType::BlackHole => {
            star.age = (num2 * 0.400000005960464 + 1.0) as f32;
            if need_type == StarType::WhiteDwarf {
                star.lifetime += 10000.0;
            } else if need_type == StarType::NeutronStar {
                star.lifetime += 1000.0;
            }
        }
        _ => {
            star.age = if star.mass >= 0.8 {
                num2 * 0.699999988079071 + 0.200000002980232
            } else if star.mass >= 0.5 {
                num2 * 0.400000005960464 + 0.100000001490116
            } else {
                num2 * 0.119999997317791 + 0.0199999995529652
            } as f32;
        }
    };

    let mut num9 = star.lifetime * star.age;
    if num9 > 5000.0 {
        num9 = (((num9 / 5000.0).ln() as f64 + 1.0) * 5000.0) as f32;
    }
    if num9 > 8000.0 {
        num9 = (((((num9 / 8000.0).ln() + 1.0).ln() + 1.0).ln() as f64 + 1.0) * 8000.0) as f32;
    }
    star.lifetime = num9 / star.age;
    let f1 = ((1.0 - (star.age.clamp(0.0, 1.0).powf(20.0) as f64) * 0.5) as f32) * star.mass;
    star.temperature =
        ((f1 as f64).powf(0.56 + 0.14 / (((f1 as f64) + 4.0).log10() / 5_f64.log10())) * 4450.0
            + 1300.0) as f32;

    let mut num10 = (((star.temperature as f64) - 1300.0) / 4500.0).log10() / 2.6_f64.log10() - 0.5;
    if num10 < 0.0 {
        num10 *= 4.0;
    }
    num10 = num10.clamp(-4.0, 2.0);
    star.spectr = unsafe { ::std::mem::transmute((num10 + 4.0).round() as i32) };
    star.color = (((num10 + 3.5) * 0.200000002980232) as f32).clamp(0.0, 1.0);
    star.class_factor = num10 as f32;
    star.luminosity = f1.powf(0.7);
    star.radius = ((star.mass as f64).powf(0.4) * num5) as f32;
    let p2 = (num10 as f32) + 2.0;
    star.light_balance_radius = 1.7_f32.powf(p2);
    star.habitable_radius = star.light_balance_radius + 0.25;
    star.orbit_scaler = 1.35_f32.powf(p2);
    if star.orbit_scaler < 1.0 {
        star.orbit_scaler += (1.0 - star.orbit_scaler) * 0.6;
    }
    let age = star.age;
    set_star_age(&mut star, age, rn, rt);
    star.dyson_radius = star.orbit_scaler * 0.28;
    if star.dyson_radius * 40000.0 < star.physics_radius() * 1.5 {
        star.dyson_radius = star.physics_radius() * 1.5 / 40000.0;
    }
    star
}

pub fn create_birth_star(seed: i32) -> Star {
    let mut star = Star::new();
    star.index = 0;
    star.level = 0.0;
    star.id = 1;
    star.seed = seed;
    star.resource_coef = 0.6;
    let mut rand1 = DspRandom::new(seed);
    star.name_seed = rand1.next_seed();
    let mut rand2 = DspRandom::new(rand1.next_seed());
    let r1_1 = rand2.next_f64();
    let r2_1 = rand2.next_f64();
    let num1 = rand2.next_f64();
    let rn = rand2.next_f64();
    let rt = rand2.next_f64();
    let num2 = rand2.next_f64() * 0.2 + 0.9;
    let num3 = 2_f64.powf(rand2.next_f64() * 0.4 - 0.2);
    let p1 = rand_normal(0.0, 0.08, r1_1, r2_1).clamp(-0.2, 0.2);
    star.mass = 2_f32.powf(p1);
    let d = 2.0 + 0.4 * (1.0 - (star.mass as f64));
    star.lifetime = (10000.0
        * 0.1_f64.powf(((star.mass as f64) * 0.5).log10() / d.log10() + 1.0)
        * num2) as f32;
    star.age = (num1 * 0.4 + 0.3) as f32;
    let f1 = ((1.0 - (star.age.clamp(0.0, 1.0).powf(20.0) as f64) * 0.5) as f32) * star.mass;
    star.temperature =
        ((f1 as f64).powf(0.56 + 0.14 / (((f1 as f64) + 4.0).log10() / 5_f64.log10())) * 4450.0
            + 1300.0) as f32;

    let mut num5 = (((star.temperature as f64) - 1300.0) / 4500.0).log10() / 2.6_f64.log10() - 0.5;
    if num5 < 0.0 {
        num5 *= 4.0;
    }
    num5 = num5.clamp(-4.0, 2.0);
    star.spectr = unsafe { ::std::mem::transmute((num5 + 4.0).round() as i32) };
    star.color = (((num5 + 3.5) * 0.200000002980232) as f32).clamp(0.0, 1.0);
    star.class_factor = num5 as f32;
    star.luminosity = f1.powf(0.7);
    star.radius = ((star.mass as f64).powf(0.4) * num3) as f32;
    let p2 = (num5 as f32) + 2.0;
    star.light_balance_radius = 1.7_f32.powf(p2);
    star.habitable_radius = star.light_balance_radius + 0.2;
    star.orbit_scaler = 1.35_f32.powf(p2);
    if star.orbit_scaler < 1.0 {
        star.orbit_scaler += (1.0 - star.orbit_scaler) * 0.6;
    }
    let age = star.age;
    set_star_age(&mut star, age, rn, rt);
    star.dyson_radius = star.orbit_scaler * 0.28;
    if star.dyson_radius * 40000.0 < star.physics_radius() * 1.5 {
        star.dyson_radius = star.physics_radius() * 1.5 / 40000.0;
    }

    // displayed luminosity
    star.luminosity = (star.luminosity.powf(0.330000013113022) * 1000.0) / 1000.0;
    star
}

pub fn create_star_planets(star: &Star, star_count: i32, habitable_count: &mut i32) -> Vec<Planet> {
    let mut rand1 = DspRandom::new(star.seed);
    rand1.next_f64();
    rand1.next_f64();
    rand1.next_f64();
    let mut rand2 = DspRandom::new(rand1.next_seed());
    let num1 = rand2.next_f64();
    let num2 = rand2.next_f64();
    let num3 = if rand2.next_f64() > 0.5 { 1 } else { 0 };
    rand2.next_f64();
    rand2.next_f64();
    rand2.next_f64();
    rand2.next_f64();

    let mut make_planet = |index: i32,
                           orbit_around_planet: Option<&Planet>,
                           orbit_index: i32,
                           gas_giant: bool|
     -> Planet {
        let info_seed = rand2.next_seed();
        let gen_seed = rand2.next_seed();
        create_planet(
            star,
            star_count,
            index,
            orbit_around_planet,
            orbit_index,
            gas_giant,
            habitable_count,
            info_seed,
            gen_seed,
        )
    };

    if star.star_type == StarType::BlackHole || star.star_type == StarType::NeutronStar {
        vec![make_planet(0, None, 3, false)]
    } else if star.star_type == StarType::WhiteDwarf {
        if num1 < 0.699999988079071 {
            vec![make_planet(0, None, 3, false)]
        } else if num2 < 0.300000011920929 {
            let planet1 = make_planet(0, None, 3, false);
            let planet2 = make_planet(1, None, 4, false);
            vec![planet1, planet2]
        } else {
            let planet1 = make_planet(0, None, 4, true);
            let planet2 = make_planet(1, Some(&planet1), 1, false);
            vec![planet1, planet2]
        }
    } else if star.star_type == StarType::GiantStar {
        if num1 < 0.300000011920929 {
            vec![make_planet(0, None, 2 + num3, false)]
        } else if num1 < 0.800000011920929 {
            if num2 < 0.25 {
                let planet1 = make_planet(0, None, 2 + num3, false);
                let planet2 = make_planet(1, None, 3 + num3, false);
                vec![planet1, planet2]
            } else {
                let planet1 = make_planet(0, None, 3, true);
                let planet2 = make_planet(1, Some(&planet1), 1, false);
                vec![planet1, planet2]
            }
        } else {
            if num2 < 0.150000005960464 {
                let planet1 = make_planet(0, None, 2 + num3, false);
                let planet2 = make_planet(1, None, 3 + num3, false);
                let planet3 = make_planet(2, None, 4 + num3, false);
                vec![planet1, planet2, planet3]
            } else if num2 < 0.75 {
                let planet1 = make_planet(0, None, 2 + num3, false);
                let planet2 = make_planet(1, None, 4, true);
                let planet3 = make_planet(2, Some(&planet2), 1, false);
                vec![planet1, planet2, planet3]
            } else {
                let planet1 = make_planet(0, None, 3 + num3, true);
                let planet2 = make_planet(1, Some(&planet1), 1, false);
                let planet3 = make_planet(2, Some(&planet1), 2, false);
                vec![planet1, planet2, planet3]
            }
        }
    } else {
        let (planet_count, p_gas): (i32, [f64; 6]) = if star.index == 0 {
            (4, P_GASES[0])
        } else if star.spectr == SpectrType::M {
            let planet_count = if num1 >= 0.8 {
                4
            } else if num1 >= 0.3 {
                3
            } else if num1 >= 0.1 {
                2
            } else {
                1
            };
            (
                planet_count,
                if planet_count <= 3 {
                    P_GASES[1]
                } else {
                    P_GASES[2]
                },
            )
        } else if star.spectr == SpectrType::K {
            let planet_count = if num1 >= 0.95 {
                5
            } else if num1 >= 0.7 {
                4
            } else if num1 >= 0.2 {
                3
            } else if num1 >= 0.1 {
                2
            } else {
                1
            };
            (
                planet_count,
                if planet_count <= 3 {
                    P_GASES[3]
                } else {
                    P_GASES[4]
                },
            )
        } else if star.spectr == SpectrType::G {
            let planet_count = if num1 >= 0.9 {
                5
            } else if num1 >= 0.4 {
                4
            } else {
                3
            };
            (
                planet_count,
                if planet_count <= 3 {
                    P_GASES[3]
                } else {
                    P_GASES[5]
                },
            )
        } else if star.spectr == SpectrType::F {
            let planet_count = if num1 >= 0.8 {
                5
            } else if num1 >= 0.35 {
                4
            } else {
                3
            };
            (
                planet_count,
                if planet_count <= 3 {
                    P_GASES[1]
                } else {
                    P_GASES[6]
                },
            )
        } else if star.spectr == SpectrType::A {
            let planet_count = if num1 >= 0.75 {
                5
            } else if num1 >= 0.3 {
                4
            } else {
                3
            };
            (
                planet_count,
                if planet_count <= 3 {
                    P_GASES[1]
                } else {
                    P_GASES[7]
                },
            )
        } else if star.spectr == SpectrType::B {
            let planet_count = if num1 >= 0.75 {
                6
            } else if num1 >= 0.3 {
                5
            } else {
                4
            };
            (
                planet_count,
                if planet_count <= 3 {
                    P_GASES[1]
                } else {
                    P_GASES[8]
                },
            )
        } else if star.spectr == SpectrType::O {
            let planet_count = if num1 >= 0.5 { 6 } else { 5 };
            (planet_count, P_GASES[9])
        } else {
            (1, P_GASES[0])
        };
        let mut num8 = 0;
        let mut num9 = 0;
        let mut orbit_around = 0;
        let mut num10 = 1;
        let mut output: Vec<Planet> = vec![];
        for index in 0..planet_count {
            let info_seed = rand2.next_seed();
            let gen_seed = rand2.next_seed();
            let num11 = rand2.next_f64();
            let num12 = rand2.next_f64();
            let mut gas_giant = false;
            if orbit_around == 0 {
                num8 += 1;
                if index < planet_count - 1 && num11 < p_gas[index as usize] {
                    gas_giant = true;
                    if num10 < 3 {
                        num10 = 3;
                    }
                }
                let mut broke_from_loop = false;
                while star.index != 0 || num10 != 3 {
                    let num13 = planet_count - index;
                    let num14 = 9 - (num10 as i32);
                    if num14 > num13 {
                        let a = (num13 as f32) / (num14 as f32);
                        let a2 = if num10 <= 3 { 0.15_f32 } else { 0.45_f32 };
                        let num15 = a + (1.0 - a) * a2 + 0.01;
                        if rand2.next_f64() < num15 as f64 {
                            broke_from_loop = true;
                            break;
                        }
                    } else {
                        broke_from_loop = true;
                        break;
                    }
                    num10 += 1;
                }
                if !broke_from_loop {
                    gas_giant = true;
                }
            } else {
                num9 += 1;
            }
            output.push(create_planet(
                star,
                star_count,
                index,
                if orbit_around == 0 {
                    None
                } else {
                    output.get((orbit_around - 1) as usize)
                },
                if orbit_around == 0 { num10 } else { num9 },
                gas_giant,
                habitable_count,
                info_seed,
                gen_seed,
            ));
            num10 += 1;
            if gas_giant {
                orbit_around = num8;
                num9 = 0;
            }
            if num9 >= 1 && num12 < 0.8 {
                orbit_around = 0;
                num9 = 0;
            }
        }
        output
    }
}

const PI: f64 = 3.14159265358979;
const P_GASES: [[f64; 6]; 10] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],     // birth
    [0.2, 0.2, 0.0, 0.0, 0.0, 0.0],     // M / F / A / B, n <= 3
    [0.0, 0.2, 0.3, 0.0, 0.0, 0.0],     // M, n >= 4
    [0.18, 0.18, 0.0, 0.0, 0.0, 0.0],   // K / G, n <= 3
    [0.0, 0.18, 0.28, 0.28, 0.0, 0.0],  // K, n >= 4
    [0.0, 0.2, 0.3, 0.3, 0.0, 0.0],     // G, n >= 4
    [0.0, 0.22, 0.31, 0.31, 0.0, 0.0],  // F, n >= 4
    [0.1, 0.28, 0.3, 0.35, 0.0, 0.0],   // A, n >= 4
    [0.1, 0.22, 0.28, 0.35, 0.35, 0.0], // B, n >= 4
    [0.1, 0.2, 0.25, 0.3, 0.32, 0.35],  // O
];

fn rand_normal(average_value: f32, standard_deviation: f32, r1: f64, r2: f64) -> f32 {
    average_value
        + standard_deviation * ((-2.0 * (1.0 - r1).ln()).sqrt() * (2.0 * PI * r2).sin()) as f32
}

fn set_star_age(star: &mut Star, age: f32, rn: f64, rt: f64) {
    let num1 = (rn * 0.1 + 0.95) as f32;
    let num2 = (rt * 0.4 + 0.8) as f32;
    let num3 = (rt * 9.0 + 1.0) as f32;
    star.age = age;
    if age >= 1.0 {
        if star.mass >= 18.0 {
            star.star_type = StarType::BlackHole;
            star.spectr = SpectrType::X;
            star.mass *= 2.5 * num2;
            star.temperature = 0.0;
            star.luminosity *= 1.0 / 1000.0 * num1;
            star.habitable_radius = 0.0;
            star.light_balance_radius *= 0.4 * num1;
            star.color = 1.0;
        } else if star.mass >= 7.0 {
            star.star_type = StarType::NeutronStar;
            star.spectr = SpectrType::X;
            star.mass *= 0.2 * num1;
            star.radius *= 0.15;
            star.temperature = num3 * 1e+7;
            star.luminosity *= 0.1 * num1;
            star.habitable_radius = 0.0;
            star.light_balance_radius *= 3.0 * num1;
            star.orbit_scaler *= 1.5 * num1;
            star.color = 1.0;
        } else {
            star.star_type = StarType::WhiteDwarf;
            star.spectr = SpectrType::X;
            star.mass *= 0.2 * num1;
            star.radius *= 0.2;
            star.temperature = num2 * 150000.0;
            star.luminosity *= 0.04 * num2;
            star.habitable_radius *= 0.15 * num2;
            star.light_balance_radius *= 0.2 * num1;
            star.color = 0.7;
        }
    } else {
        if age >= 0.959999978542328 {
            let mut num4 = (5.0_f64.powf(((star.mass as f64).log10() - 0.7).abs()) * 5.0) as f32;
            if num4 > 10.0 {
                num4 = ((num4 * 0.1).ln() + 1.0) * 10.0;
            }
            let num5 = 1.0 - star.age.powf(30.0) * 0.5;
            star.star_type = StarType::GiantStar;
            star.mass = num5 * star.mass;
            star.radius = num4 * num2;
            star.temperature *= num5;
            star.luminosity *= 1.6;
            star.habitable_radius *= 9.0;
            star.light_balance_radius = 3.0 * star.habitable_radius;
            star.orbit_scaler *= 3.3;
        }
    }
}
