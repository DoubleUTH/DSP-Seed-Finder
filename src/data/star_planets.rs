use std::cell::{Cell, UnsafeCell};
use std::rc::Rc;

use crate::data::game_desc::GameDesc;

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

const MAX_VEIN_COUNT: usize = VeinType::Max as usize;

#[derive(Debug, Serialize)]
pub struct StarWithPlanets<'a> {
    pub name: String,
    #[serde(flatten)]
    pub star: Rc<Star<'a>>,
    #[serde(serialize_with = "serialize_planets")]
    planets: UnsafeCell<Vec<Planet<'a>>>,

    #[serde(skip)]
    safe: UnsafeCell<bool>,
    #[serde(skip)]
    avg_veins: UnsafeCell<[f32; MAX_VEIN_COUNT]>,
    #[serde(skip)]
    actual_veins: UnsafeCell<[f32; MAX_VEIN_COUNT]>,
    #[serde(skip)]
    game_desc: &'a GameDesc,
    #[serde(skip)]
    habitable_count: &'a Cell<i32>,
}

impl<'a> StarWithPlanets<'a> {
    pub fn new(
        star: Rc<Star<'a>>,
        game_desc: &'a GameDesc,
        habitable_count: &'a Cell<i32>,
    ) -> Self {
        Self {
            star,
            planets: UnsafeCell::new(Vec::with_capacity(6)),
            safe: UnsafeCell::new(false),
            avg_veins: UnsafeCell::new([f32::NAN; MAX_VEIN_COUNT]),
            actual_veins: UnsafeCell::new([f32::NAN; MAX_VEIN_COUNT]),
            name: Default::default(),
            game_desc,
            habitable_count,
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
        let index = *vein_type as usize;
        let cached_value = unsafe {
            let arr = &mut *self.avg_veins.get();
            arr.get_unchecked_mut(index)
        };
        if !cached_value.is_nan() {
            return *cached_value;
        }
        let mut count = 0_f32;
        for planet in self.get_planets() {
            if !planet.can_have_vein(vein_type) {
                continue;
            }
            if planet.is_acutal_veins_generated() {
                for vein in planet.get_actual_veins() {
                    if &vein.vein_type == vein_type {
                        count += vein.amount as f32;
                    }
                }
            } else {
                for vein in planet.get_estimated_veins() {
                    if &vein.vein_type == vein_type {
                        let avg_patches = ((vein.min_patch + vein.max_patch) as f32)
                            * ((vein.min_group + vein.max_group) as f32)
                            * ((vein.min_amount + vein.max_amount) as f32)
                            / 8.0;
                        count += avg_patches;
                    }
                }
            }
        }
        *cached_value = count;
        self.mark_safe();
        count
    }

    pub fn get_actual_vein(&self, vein_type: &VeinType) -> f32 {
        if vein_type == &VeinType::Mag
            && self.star.star_type != StarType::BlackHole
            && self.star.star_type != StarType::NeutronStar
        {
            return 0.0;
        }
        let index = *vein_type as usize;
        let cached_value = unsafe {
            let arr = &mut *self.actual_veins.get();
            arr.get_unchecked_mut(index)
        };
        if !cached_value.is_nan() {
            return *cached_value;
        }
        let mut count = 0;
        for planet in self.get_planets() {
            if !planet.can_have_vein(vein_type) {
                continue;
            }
            for vein in planet.get_actual_veins() {
                if &vein.vein_type == vein_type {
                    count += vein.amount;
                }
            }
        }
        *cached_value = count as f32;
        self.mark_safe();
        count as f32
    }

    pub fn get_planets(&self) -> &Vec<Planet<'a>> {
        let planets = unsafe { &mut *self.planets.get() };
        if !planets.is_empty() {
            return planets;
        }
        let mut rand2 = DspRandom::new(self.star.planets_seed);
        let planet_count_rand = rand2.next_f64();
        let planet_config_rand = rand2.next_f64();
        let orbit_offset = if rand2.next_f64() > 0.5 { 1 } else { 0 };
        rand2.next_f64();
        rand2.next_f64();
        rand2.next_f64();
        rand2.next_f64();

        let mut make_planet = |index: usize, orbit_index: usize, gas_giant: bool| -> Planet {
            let info_seed = rand2.next_seed();
            let gen_seed = rand2.next_seed();
            Planet::new(
                self.game_desc,
                self.star.clone(),
                index,
                self.habitable_count,
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
            if planet_count_rand < 0.7 {
                planets.push(make_planet(0, 3, false));
            } else if planet_config_rand < 0.3 {
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
            if planet_count_rand < 0.3 {
                planets.push(make_planet(0, 2 + orbit_offset, false));
            } else if planet_count_rand < 0.8 {
                if planet_config_rand < 0.25 {
                    planets.push(make_planet(0, 2 + orbit_offset, false));
                    planets.push(make_planet(1, 3 + orbit_offset, false));
                } else {
                    planets.push(make_planet(0, 3, true));
                    planets.push(make_planet(1, 1, false));
                    let planet1 = &planets[0];
                    let planet2 = &planets[1];
                    planet2.orbit_around.replace(Some(planet1));
                }
            } else {
                if planet_config_rand < 0.15 {
                    planets.push(make_planet(0, 2 + orbit_offset, false));
                    planets.push(make_planet(1, 3 + orbit_offset, false));
                    planets.push(make_planet(2, 4 + orbit_offset, false));
                } else if planet_config_rand < 0.75 {
                    planets.push(make_planet(0, 2 + orbit_offset, false));
                    planets.push(make_planet(1, 4, true));
                    planets.push(make_planet(2, 1, false));
                    let planet2 = &planets[1];
                    let planet3 = &planets[2];
                    planet3.orbit_around.replace(Some(planet2));
                } else {
                    planets.push(make_planet(0, 3 + orbit_offset, true));
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
                        let planet_count = if planet_count_rand >= 0.8 {
                            4
                        } else if planet_count_rand >= 0.3 {
                            3
                        } else if planet_count_rand >= 0.1 {
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
                        let planet_count = if planet_count_rand >= 0.95 {
                            5
                        } else if planet_count_rand >= 0.7 {
                            4
                        } else if planet_count_rand >= 0.2 {
                            3
                        } else if planet_count_rand >= 0.1 {
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
                        let planet_count = if planet_count_rand >= 0.9 {
                            5
                        } else if planet_count_rand >= 0.4 {
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
                        let planet_count = if planet_count_rand >= 0.8 {
                            5
                        } else if planet_count_rand >= 0.35 {
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
                        let planet_count = if planet_count_rand >= 0.75 {
                            5
                        } else if planet_count_rand >= 0.3 {
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
                        let planet_count = if planet_count_rand >= 0.75 {
                            6
                        } else if planet_count_rand >= 0.3 {
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
                        let planet_count = if planet_count_rand >= 0.5 { 6 } else { 5 };
                        (planet_count, P_GASES[9])
                    }
                    _ => (1, P_GASES[0]),
                }
            };
            let mut satellite_count = 0;
            let mut orbit_around: Option<usize> = None;
            let mut current_orbit_index: usize = 1;
            let mut orbits: Vec<(usize, usize)> = Vec::with_capacity(4);
            for index in 0..planet_count as usize {
                let info_seed = rand2.next_seed();
                let gen_seed = rand2.next_seed();
                let gas_giant_chance_rand = rand2.next_f64();
                let stop_satellite_chance_rand = rand2.next_f64();
                let mut gas_giant = false;

                if orbit_around.is_none() {
                    if index < planet_count - 1 && gas_giant_chance_rand < p_gas[index] {
                        gas_giant = true;
                        if current_orbit_index < 3 {
                            current_orbit_index = 3;
                        }
                    }
                    let mut broke_from_loop = false;
                    while !self.star.is_birth() || current_orbit_index != 3 {
                        let remaining_planets = planet_count - index;
                        let remaining_orbit_slots = 9 - current_orbit_index;
                        if remaining_orbit_slots > remaining_planets {
                            let remaining_ratio =
                                (remaining_planets as f32) / (remaining_orbit_slots as f32);
                            let skip_chance_base = if current_orbit_index <= 3 {
                                0.15_f32
                            } else {
                                0.45_f32
                            };
                            let orbit_skip_threshold =
                                remaining_ratio + (1.0 - remaining_ratio) * skip_chance_base + 0.01;
                            if rand2.next_f64() < orbit_skip_threshold as f64 {
                                broke_from_loop = true;
                                break;
                            }
                        } else {
                            broke_from_loop = true;
                            break;
                        }
                        current_orbit_index += 1;
                    }
                    if !broke_from_loop {
                        gas_giant = true;
                    }
                } else {
                    satellite_count += 1;
                }
                let planet = Planet::new(
                    self.game_desc,
                    self.star.clone(),
                    index,
                    self.habitable_count,
                    if orbit_around.is_none() {
                        current_orbit_index
                    } else {
                        satellite_count
                    },
                    gas_giant,
                    info_seed,
                    gen_seed,
                );
                if let Some(around) = orbit_around {
                    orbits.push((index, around))
                }
                current_orbit_index += 1;
                if gas_giant {
                    orbit_around = Some(index);
                    satellite_count = 0;
                }
                if satellite_count >= 1 && stop_satellite_chance_rand < 0.8 {
                    orbit_around = None;
                    satellite_count = 0;
                }
                planets.push(planet);
            }
            for (index, orbit_index) in orbits {
                let planet = &planets[index];
                let orbit_planet = &planets[orbit_index];
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
