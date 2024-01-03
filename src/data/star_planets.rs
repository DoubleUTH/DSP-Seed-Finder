use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::enums::{SpectrType, StarType, VeinType};
use super::planet::Planet;
use super::random::DspRandom;
use super::star::Star;
use serde::Serialize;

pub fn serialize_planets<S>(
    planets: &UnsafeCell<Vec<Planet<'_>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    unsafe { &*planets.get() }.serialize(serializer)
}

#[derive(Debug, Serialize)]
pub struct StarWithPlanets<'a> {
    #[serde(flatten)]
    pub star: Rc<Star<'a>>,
    #[serde(serialize_with = "serialize_planets")]
    planets: UnsafeCell<Vec<Planet<'a>>>,
    #[serde(skip)]
    safe: UnsafeCell<bool>,
    #[serde(skip)]
    avg_veins: UnsafeCell<HashMap<VeinType, f32>>,
    pub name: String,
}

impl<'a> StarWithPlanets<'a> {
    pub fn new(star: Rc<Star<'a>>) -> Self {
        Self {
            star,
            planets: UnsafeCell::new(vec![]),
            safe: UnsafeCell::new(false),
            avg_veins: UnsafeCell::new(HashMap::new()),
            name: Default::default(),
        }
    }

    pub fn is_safe(&self) -> bool {
        unsafe { *self.safe.get() }
    }

    pub fn mark_safe(&self) {
        unsafe {
            *self.safe.get() = true;
        }
    }

    pub fn load_planets(&self) {
        for p in self.get_planets() {
            // load the data
            p.get_theme();
        }
        self.mark_safe();
    }

    pub fn get_avg_vein(&self, vein_type: &VeinType) -> f32 {
        if vein_type == &VeinType::Mag
            && self.star.star_type != StarType::BlackHole
            && self.star.star_type != StarType::NeutronStar
        {
            return 0.0;
        }
        let map = unsafe { &mut *self.avg_veins.get() };
        if let Some(val) = map.get(vein_type) {
            return *val;
        }
        let mut count = 0_f32;
        let is_rare = vein_type.is_rare();
        let is_safe = self.is_safe();
        for planet in self.get_planets() {
            if planet.is_gas_giant() {
                if !is_safe {
                    planet.get_theme();
                }
                continue;
            }
            if is_rare {
                let theme = planet.get_theme();
                // skip vein generation if possible
                if !theme.rare_veins.contains(vein_type) {
                    continue;
                }
            }
            for vein in planet.get_veins() {
                if &vein.vein_type == vein_type {
                    let avg_patches = ((vein.min_patch + vein.max_patch) as f32)
                        * ((vein.min_group + vein.max_group) as f32)
                        * ((vein.min_amount + vein.max_amount) as f32)
                        / 8.0;
                    count += avg_patches;
                }
            }
        }
        map.insert(vein_type.clone(), count);
        self.mark_safe();
        count
    }

    pub fn get_planets(&self) -> &Vec<Planet<'a>> {
        let planets = unsafe { &mut *self.planets.get() };
        if !planets.is_empty() {
            return planets;
        }
        let mut rand2 = DspRandom::new(self.star.planets_seed);
        let num1 = rand2.next_f64();
        let num2 = rand2.next_f64();
        let num3 = if rand2.next_f64() > 0.5 { 1 } else { 0 };
        rand2.next_f64();
        rand2.next_f64();
        rand2.next_f64();
        rand2.next_f64();

        let mut make_planet = |index: usize, orbit_index: usize, gas_giant: bool| -> Planet {
            let info_seed = rand2.next_seed();
            let gen_seed = rand2.next_seed();
            Planet::new(
                self.star.clone(),
                index,
                orbit_index,
                gas_giant,
                info_seed,
                gen_seed,
            )
        };

        let star_type = &self.star.star_type;

        if star_type == &StarType::BlackHole || star_type == &StarType::NeutronStar {
            planets.push(make_planet(0, 3, false));
        } else if star_type == &StarType::WhiteDwarf {
            if num1 < 0.7 {
                planets.push(make_planet(0, 3, false));
            } else if num2 < 0.3 {
                planets.push(make_planet(0, 3, false));
                planets.push(make_planet(1, 4, false));
            } else {
                planets.push(make_planet(0, 4, true));
                planets.push(make_planet(1, 1, false));
                let planet1 = &planets[0];
                let planet2 = &planets[1];
                planet2.orbit_around.replace(Some(planet1));
            }
        } else if star_type == &StarType::GiantStar {
            if num1 < 0.3 {
                planets.push(make_planet(0, 2 + num3, false));
            } else if num1 < 0.8 {
                if num2 < 0.25 {
                    planets.push(make_planet(0, 2 + num3, false));
                    planets.push(make_planet(1, 3 + num3, false));
                } else {
                    planets.push(make_planet(0, 3, true));
                    planets.push(make_planet(1, 1, false));
                    let planet1 = &planets[0];
                    let planet2 = &planets[1];
                    planet2.orbit_around.replace(Some(planet1));
                }
            } else {
                if num2 < 0.15 {
                    planets.push(make_planet(0, 2 + num3, false));
                    planets.push(make_planet(1, 3 + num3, false));
                    planets.push(make_planet(2, 4 + num3, false));
                } else if num2 < 0.75 {
                    planets.push(make_planet(0, 2 + num3, false));
                    planets.push(make_planet(1, 4, true));
                    planets.push(make_planet(2, 1, false));
                    let planet2 = &planets[1];
                    let planet3 = &planets[2];
                    planet3.orbit_around.replace(Some(planet2));
                } else {
                    planets.push(make_planet(0, 3 + num3, true));
                    planets.push(make_planet(1, 1, false));
                    planets.push(make_planet(2, 2, false));
                    let planet1 = &planets[0];
                    let planet2 = &planets[1];
                    let planet3 = &planets[2];
                    planet2.orbit_around.replace(Some(planet1));
                    planet3.orbit_around.replace(Some(planet1));
                }
            }
        } else {
            let (planet_count, p_gas): (usize, [f64; 6]) = if self.star.is_birth() {
                (4, P_GASES[0])
            } else {
                match self.star.get_spectr() {
                    SpectrType::M => {
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
                    }
                    SpectrType::K => {
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
                    }
                    SpectrType::G => {
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
                    }
                    SpectrType::F => {
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
                    }
                    SpectrType::A => {
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
                    }
                    SpectrType::B => {
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
                    }
                    SpectrType::O => {
                        let planet_count = if num1 >= 0.5 { 6 } else { 5 };
                        (planet_count, P_GASES[9])
                    }
                    _ => (1, P_GASES[0]),
                }
            };
            let mut num8 = 0;
            let mut num9 = 0;
            let mut orbit_around: usize = 0;
            let mut num10: usize = 1;
            let mut orbits: Vec<(usize, usize)> = vec![];
            for index in 0..planet_count as usize {
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
                    while !self.star.is_birth() || num10 != 3 {
                        let num13 = planet_count - index;
                        let num14 = 9 - num10;
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
                let planet = Planet::new(
                    self.star.clone(),
                    index,
                    if orbit_around == 0 { num10 } else { num9 },
                    gas_giant,
                    info_seed,
                    gen_seed,
                );
                if orbit_around > 0 {
                    orbits.push((index, orbit_around - 1))
                }
                num10 += 1;
                if gas_giant {
                    orbit_around = num8;
                    num9 = 0;
                }
                if num9 >= 1 && num12 < 0.8 {
                    orbit_around = 0;
                    num9 = 0;
                }
                planets.push(planet);
            }
            for (index, orbit_index) in orbits {
                let planet = &planets[index as usize];
                let orbit_planet = &planets[orbit_index as usize];
                planet.orbit_around.replace(Some(orbit_planet));
            }
        }

        planets
    }
}

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
