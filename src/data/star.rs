use super::enums::{SpectrType, StarType};
use super::game_desc::GameDesc;
use super::random::DspRandom;
use super::vector3::Vector3;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::cell::{OnceCell, RefCell};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Star<'a> {
    pub game_desc: &'a GameDesc,
    pub used_theme_ids: RefCell<Vec<i32>>,
    pub index: usize,
    pub name_seed: i32,
    pub position: Vector3,
    pub level: f32,
    pub star_type: StarType,
    age_factor: f64,
    age_num1: f32,
    age_num2: f32,
    age_num3: f32,
    lifetime_factor: f64,
    radius_factor: f64,
    pub planets_seed: i32,
    safety_factor_modifier: f64,
    max_hive_count_modifier: i32,
    mass_params: (f64, f64, f64, f64, f32),
    unmodified_mass: OnceCell<f32>,
    resource_coef: OnceCell<f32>,
    age: OnceCell<f32>,
    temperature_factor: OnceCell<f32>,
    unmodified_temperature: OnceCell<f32>,
    temperature: OnceCell<f32>,
    class_factor: OnceCell<f64>,
    spectr: OnceCell<SpectrType>,
    luminosity: OnceCell<f32>,
    radius: OnceCell<f32>,
    light_balance_radius: OnceCell<f32>,
    habitable_radius: OnceCell<f32>,
    mass: OnceCell<f32>,
    orbit_scaler: OnceCell<f32>,
    dyson_radius: OnceCell<i32>,
    hive_rand: RefCell<DspRandom>,
    max_hive_count: OnceCell<i32>,
    initial_hive_count: OnceCell<i32>,
}

impl<'a> Star<'a> {
    pub fn new(
        game_desc: &'a GameDesc,
        index: usize,
        seed: i32,
        position: Vector3,
        need_type: StarType,
        need_spectr: &SpectrType,
    ) -> Self {
        let mut rand1 = DspRandom::new(seed);
        let name_seed = rand1.next_seed();
        let mut rand2 = DspRandom::new(rand1.next_seed());
        rand1.next_f64();
        let planets_seed = rand1.next_seed();
        let mass_random1 = rand2.next_f64();
        let mass_random2 = rand2.next_f64();
        let age_factor = rand2.next_f64();
        let age_num1_rand = rand2.next_f64();
        let age_factor_rand = rand2.next_f64();
        let age_num1 = (age_num1_rand * 0.1 + 0.95) as f32;
        let age_num2 = (age_factor_rand * 0.4 + 0.8) as f32;
        let age_num3 = (age_factor_rand * 9.0 + 1.0) as f32;
        let mass_factor = if index == 0 { 0.0 } else { rand2.next_f64() };
        let lifetime_factor = rand2.next_f64();
        let radius_exponent = rand2.next_f64() * 0.4 - 0.2;
        let radius_factor = 2_f64.powf(radius_exponent);
        let hive_seed = rand2.next_seed();
        let mut hive_rand = DspRandom::new(hive_seed);
        let safety_factor_modifier = hive_rand.next_f64();
        let max_hive_count_modifier = hive_rand.next_i32(1000);
        let mass_params = (
            mass_random1,
            mass_random2,
            radius_exponent,
            mass_factor,
            match need_spectr {
                SpectrType::M => -3_f32,
                SpectrType::O => 4.65_f32,
                _ => 0.0,
            },
        );

        Self {
            game_desc,
            used_theme_ids: RefCell::new(vec![]),
            index,
            name_seed,
            position,
            level: (index as f32) / ((game_desc.star_count - 1) as f32),
            star_type: need_type,
            age_factor,
            age_num1,
            age_num2,
            age_num3,
            lifetime_factor,
            radius_factor,
            planets_seed,
            mass_params,
            safety_factor_modifier,
            max_hive_count_modifier,
            unmodified_mass: OnceCell::new(),
            resource_coef: OnceCell::new(),
            age: OnceCell::new(),
            temperature_factor: OnceCell::new(),
            unmodified_temperature: OnceCell::new(),
            temperature: OnceCell::new(),
            class_factor: OnceCell::new(),
            spectr: OnceCell::new(),
            luminosity: OnceCell::new(),
            radius: OnceCell::new(),
            light_balance_radius: OnceCell::new(),
            habitable_radius: OnceCell::new(),
            mass: OnceCell::new(),
            orbit_scaler: OnceCell::new(),
            dyson_radius: OnceCell::new(),
            hive_rand: RefCell::new(hive_rand),
            max_hive_count: OnceCell::new(),
            initial_hive_count: OnceCell::new(),
        }
    }

    pub fn is_birth(&self) -> bool {
        return self.index == 0;
    }

    pub fn get_unmodified_mass(&self) -> f32 {
        *self.unmodified_mass.get_or_init(|| {
            let (mass_random1, mass_random2, radius_exponent, mass_factor, spectr_factor) =
                self.mass_params;
            if self.is_birth() {
                let birth_mass_exponent =
                    rand_normal(0.0, 0.08, mass_random1, mass_random2).clamp(-0.2, 0.2);
                2_f32.powf(birth_mass_exponent)
            } else {
                match self.star_type {
                    StarType::WhiteDwarf => (1.0 + mass_random2 * 5.0) as f32,
                    StarType::NeutronStar => (7.0 + mass_random1 * 11.0) as f32,
                    StarType::BlackHole => (18.0 + mass_random1 * mass_random2 * 30.0) as f32,
                    _ => {
                        let mass_exponent = if spectr_factor != 0.0 {
                            spectr_factor
                        } else {
                            let base_spectr_exponent = lerp(-0.98, 0.88, self.level);
                            let average_value = if self.star_type == StarType::GiantStar {
                                if radius_exponent > -0.08 {
                                    -1.5
                                } else {
                                    1.6
                                }
                            } else if base_spectr_exponent >= 0.0 {
                                base_spectr_exponent + 0.65
                            } else {
                                base_spectr_exponent - 0.65
                            };
                            let standard_deviation = if self.star_type == StarType::GiantStar {
                                0.3_f32
                            } else {
                                0.33_f32
                            };
                            let random_mass_exponent = rand_normal(
                                average_value,
                                standard_deviation,
                                mass_random1,
                                mass_random2,
                            );
                            (if random_mass_exponent <= 0.0 {
                                random_mass_exponent
                            } else {
                                random_mass_exponent * 2.0
                            })
                            .clamp(-2.4, 4.65)
                        };
                        2_f32.powf((mass_exponent as f64 + (mass_factor - 0.5) * 0.2 + 1.0) as f32)
                    }
                }
            }
        })
    }

    pub fn get_resource_coef(&self) -> f32 {
        *self.resource_coef.get_or_init(|| {
            if self.is_birth() {
                0.6
            } else {
                let mut distance_factor = (self.position.magnitude() as f32) / 32.0;
                if (distance_factor as f64) > 1.0 {
                    distance_factor =
                        ((((distance_factor.ln() + 1.0).ln() + 1.0).ln() + 1.0).ln() + 1.0).ln()
                            + 1.0
                }
                7.0_f32.powf(distance_factor) * 0.6
            }
        })
    }

    fn get_lifetime(&self) -> f32 {
        let unmodified_mass = self.get_unmodified_mass();
        let lifetime_exponent_base = if unmodified_mass < 2.0 {
            2.0 + 0.4 * (1.0 - (unmodified_mass as f64))
        } else {
            5.0
        };
        let mass_multiplier = if self.star_type == StarType::GiantStar {
            0.58
        } else {
            0.5
        };
        let lifetime_delta = match self.star_type {
            StarType::WhiteDwarf => 10000.0,
            StarType::NeutronStar => 1000.0,
            _ => 0.0,
        };
        let lifetime = (10000.0
            * 0.1_f64.powf(
                ((self.get_unmodified_mass() as f64) * mass_multiplier).log(lifetime_exponent_base)
                    + 1.0,
            )
            * (self.lifetime_factor * 0.2 + 0.9))
            + lifetime_delta;

        if self.is_birth() {
            lifetime as f32
        } else {
            let age = self.get_age();
            let mut adjusted_lifetime = (lifetime as f32) * age;
            if adjusted_lifetime > 5000.0 {
                adjusted_lifetime =
                    (((adjusted_lifetime / 5000.0).ln() as f64 + 1.0) * 5000.0) as f32;
            }
            if adjusted_lifetime > 8000.0 {
                adjusted_lifetime =
                    (((((adjusted_lifetime / 8000.0).ln() + 1.0).ln() + 1.0).ln() as f64 + 1.0)
                        * 8000.0) as f32;
            }
            adjusted_lifetime / age
        }
    }

    pub fn get_age(&self) -> f32 {
        *self.age.get_or_init(|| {
            (if self.is_birth() {
                self.age_factor * 0.4 + 0.3
            } else {
                match self.star_type {
                    StarType::GiantStar => self.age_factor * 0.04 + 0.96,
                    StarType::WhiteDwarf | StarType::NeutronStar | StarType::BlackHole => {
                        self.age_factor * 0.4 + 1.0
                    }
                    _ => {
                        let unmodified_mass = self.get_unmodified_mass();
                        if unmodified_mass >= 0.8 {
                            self.age_factor * 0.7 + 0.2
                        } else if unmodified_mass >= 0.5 {
                            self.age_factor * 0.4 + 0.1
                        } else {
                            self.age_factor * 0.12 + 0.02
                        }
                    }
                }
            }) as f32
        })
    }

    pub fn get_temperature_factor(&self) -> f32 {
        *self.temperature_factor.get_or_init(|| {
            ((1.0 - (self.get_age().clamp(0.0, 1.0).powf(20.0) as f64) * 0.5) as f32)
                * self.get_unmodified_mass()
        })
    }

    pub fn get_unmodified_temperature(&self) -> f32 {
        *self.unmodified_temperature.get_or_init(|| {
            let temperature_factor_f64 = self.get_temperature_factor() as f64;
            (temperature_factor_f64.powf(0.56 + 0.14 / (temperature_factor_f64 + 4.0).log(5.0))
                * 4450.0
                + 1300.0) as f32
        })
    }

    pub fn get_temperature(&self) -> f32 {
        *self.temperature.get_or_init(|| match self.star_type {
            StarType::BlackHole => 0.0,
            StarType::NeutronStar => self.age_num3 * 1e+7,
            StarType::WhiteDwarf => self.age_num2 * 150000.0,
            _ => {
                let temperature = self.get_unmodified_temperature();
                if self.star_type == StarType::GiantStar {
                    let age_mass_factor = 1.0 - self.get_age().powf(30.0) * 0.5;
                    temperature * age_mass_factor
                } else {
                    temperature
                }
            }
        })
    }

    pub fn get_class_factor(&self) -> f64 {
        *self.class_factor.get_or_init(|| {
            let temperature = self.get_unmodified_temperature() as f64;
            let mut spectr_factor = ((temperature - 1300.0) / 4500.0).log(2.6) - 0.5;
            if spectr_factor < 0.0 {
                spectr_factor *= 4.0;
            }
            spectr_factor.clamp(-4.0, 2.0)
        })
    }

    pub fn get_spectr(&self) -> SpectrType {
        *self.spectr.get_or_init(|| {
            if matches!(
                self.star_type,
                StarType::WhiteDwarf | StarType::NeutronStar | StarType::BlackHole
            ) {
                SpectrType::X
            } else {
                unsafe {
                    ::std::mem::transmute::<i32, SpectrType>(
                        self.get_class_factor().round_ties_even() as i32,
                    )
                }
            }
        })
    }

    fn get_color(&self) -> f32 {
        match self.star_type {
            StarType::BlackHole | StarType::NeutronStar => 1.0,
            StarType::WhiteDwarf => 0.7,
            _ => (((self.get_class_factor() + 3.5) * 0.2) as f32).clamp(0.0, 1.0),
        }
    }

    pub fn get_luminosity(&self) -> f32 {
        *self.luminosity.get_or_init(|| {
            let base = self.get_temperature_factor().powf(0.7);
            let factor = match self.star_type {
                StarType::BlackHole => 1.0 / 1000.0 * self.age_num1,
                StarType::NeutronStar => 0.1 * self.age_num1,
                StarType::WhiteDwarf => 0.04 * self.age_num1,
                StarType::GiantStar => 1.6,
                _ => 1.0,
            };
            let real = base * factor;
            // displayed
            (real.powf(0.33) * 1000.0).round_ties_even() / 1000.0
        })
    }

    pub fn get_radius(&self) -> f32 {
        *self.radius.get_or_init(|| {
            if self.star_type == StarType::GiantStar {
                let mut giant_radius = (5.0_f64
                    .powf(((self.get_unmodified_mass() as f64).log10() - 0.7).abs())
                    * 5.0) as f32;
                if giant_radius > 10.0 {
                    giant_radius = ((giant_radius * 0.1).ln() + 1.0) * 10.0;
                }
                giant_radius * self.age_num2
            } else {
                (((self.get_unmodified_mass() as f64).powf(0.4) * self.radius_factor) as f32)
                    * (match self.star_type {
                        StarType::NeutronStar => 0.15,
                        StarType::WhiteDwarf => 0.2,
                        _ => 1.0,
                    })
            }
        })
    }

    pub fn get_light_balance_radius(&self) -> f32 {
        *self.light_balance_radius.get_or_init(|| {
            if self.star_type == StarType::GiantStar {
                3.0 * self.get_habitable_radius()
            } else {
                let r = 1.7_f32.powf((self.get_class_factor() as f32) + 2.0);
                let factor = match self.star_type {
                    StarType::BlackHole => 0.4 * self.age_num1,
                    StarType::NeutronStar => 3.0 * self.age_num1,
                    StarType::WhiteDwarf => 0.2 * self.age_num1,
                    _ => 1.0,
                };
                r * factor
            }
        })
    }

    pub fn get_habitable_radius(&self) -> f32 {
        *self.habitable_radius.get_or_init(|| {
            let factor = match self.star_type {
                StarType::BlackHole | StarType::NeutronStar => 0.0,
                StarType::WhiteDwarf => 0.15 * self.age_num2,
                StarType::GiantStar => 9.0,
                _ => 1.0,
            };
            if factor == 0.0 {
                0.0
            } else {
                (1.7_f32.powf((self.get_class_factor() as f32) + 2.0)
                    + if self.is_birth() { 0.2 } else { 0.25 })
                    * factor
            }
        })
    }

    pub fn get_mass(&self) -> f32 {
        *self.mass.get_or_init(|| match self.star_type {
            StarType::BlackHole => self.get_unmodified_mass() * 2.5 * self.age_num2,
            StarType::NeutronStar | StarType::WhiteDwarf => {
                self.get_unmodified_mass() * 0.2 * self.age_num1
            }
            StarType::GiantStar => {
                let age_mass_factor = 1.0 - self.get_age().powf(30.0) * 0.5;
                self.get_unmodified_mass() * age_mass_factor
            }
            _ => self.get_unmodified_mass(),
        })
    }

    pub fn get_orbit_scaler(&self) -> f32 {
        *self.orbit_scaler.get_or_init(|| {
            let mut orbit_scaler = 1.35_f32.powf((self.get_class_factor() as f32) + 2.0);
            if orbit_scaler < 1.0 {
                orbit_scaler += (1.0 - orbit_scaler) * 0.6;
            }
            orbit_scaler
                * (match self.star_type {
                    StarType::NeutronStar => 1.5 * self.age_num1,
                    StarType::GiantStar => 3.3,
                    _ => 1.0,
                })
        })
    }

    pub fn get_dyson_radius(&self) -> i32 {
        *self.dyson_radius.get_or_init(|| {
            (((self.get_orbit_scaler() * 0.28).max(self.get_radius() * 0.045) * 800.0)
                .round_ties_even() as i32)
                * 100
        })
    }

    fn get_safety_factor(&self) -> f32 {
        if self.is_birth() {
            return (0.847 + self.safety_factor_modifier * 0.026) as f32;
        }
        let mut adjusted_distance =
            (((self.position.magnitude() - 2.0) / 20.0) as f32).clamp(0.0, 2.5);
        if adjusted_distance > 1.0 {
            adjusted_distance = (adjusted_distance.ln() + 1.0).ln() + 1.0
        }
        let normalized_distance = adjusted_distance / 1.4;
        let star_type_color_factor: f32 = match self.star_type {
            StarType::BlackHole => 5.0,
            StarType::NeutronStar => 1.7,
            StarType::WhiteDwarf => 1.2,
            _ => {
                let base_color_factor = self.get_color().powf(1.3);
                if self.star_type == StarType::GiantStar {
                    base_color_factor.max(0.6)
                } else if self.get_spectr() == SpectrType::O {
                    base_color_factor + 0.05
                } else {
                    base_color_factor
                }
            }
        };
        ((1.0
            - ((star_type_color_factor * 0.9 + 0.07).powf(0.73) as f64)
                * (normalized_distance.powf(0.27) as f64)
            + self.safety_factor_modifier * 0.08
            - 0.04) as f32)
            .clamp(0.0, 1.0)
    }

    pub fn get_max_hive_count(&self) -> i32 {
        *self.max_hive_count.get_or_init(|| {
            let star_type_hive_multiplier = match self.star_type {
                StarType::BlackHole | StarType::NeutronStar => 2.0,
                _ => 1.0,
            };
            ((self.game_desc.hive_max_density * star_type_hive_multiplier * 1000.0
                + (self.max_hive_count_modifier as f64)
                + 0.5) as i32)
                / 1000
        })
    }

    pub fn get_initial_hive_count(&self) -> i32 {
        *self.initial_hive_count.get_or_init(|| {
            let initial_colonize = self.game_desc.hive_initial_colonize;
            if initial_colonize < 0.015 {
                return 0;
            }
            let max_hive_count = self.get_max_hive_count();
            if self.is_birth() {
                let birth_min_hives = if initial_colonize * (max_hive_count as f64) < 0.7 {
                    0
                } else {
                    1
                };
                let mut birth_avg_hives = 0.6 * (initial_colonize as f32) * (max_hive_count as f32);
                let mut birth_std_dev = 0.5;
                if birth_avg_hives < 1.0 {
                    birth_std_dev = ((birth_avg_hives.sqrt() as f64) * 0.29 + 0.21) as f32;
                } else if birth_avg_hives > max_hive_count as f32 {
                    birth_avg_hives = max_hive_count as f32;
                }
                let mut rand3 = self.hive_rand.borrow_mut();
                let mut initial_hive_count: i32 = -1;
                for _ in 0..17 {
                    let r1_2 = rand3.next_f64();
                    let r2_2 = rand3.next_f64();
                    initial_hive_count =
                        (rand_normal(birth_avg_hives, birth_std_dev, r1_2, r2_2) + 0.5) as i32;
                    if initial_hive_count >= 0 && initial_hive_count <= max_hive_count {
                        break;
                    }
                }
                return initial_hive_count.clamp(birth_min_hives, max_hive_count);
            }
            let hive_probability_base = ((1.0
                - (((self.get_safety_factor() * 1.05 - 0.15) as f32)
                    .clamp(0.0, 1.0)
                    .powf(0.82) as f64)
                - (((max_hive_count - 1) as f64) * 0.05))
                as f32)
                .clamp(0.0, 1.0)
                * ((1.1 - (max_hive_count as f64) * 0.1) as f32);
            let raw_probability = if initial_colonize > 1.0 {
                lerp(
                    hive_probability_base,
                    (1.0 + (initial_colonize - 1.0) * 0.2) as f32,
                    ((initial_colonize - 1.0) * 0.5) as f32,
                )
            } else {
                hive_probability_base * (initial_colonize as f32)
            };
            let star_type_adjusted_probability = match self.star_type {
                StarType::GiantStar => raw_probability * 1.2,
                StarType::WhiteDwarf => raw_probability * 1.4,
                StarType::NeutronStar => raw_probability * 1.6,
                StarType::BlackHole => raw_probability * 1.8,
                _ => {
                    if self.get_spectr() == SpectrType::O {
                        raw_probability * 1.1
                    } else {
                        raw_probability
                    }
                }
            };
            let expected_hive_count: f32 = (star_type_adjusted_probability
                * (max_hive_count as f32))
                .min((max_hive_count as f32) + 0.75);
            let hive_std_dev: f32 = if expected_hive_count <= 0.01 {
                0.0
            } else if expected_hive_count < 1.0 {
                expected_hive_count.sqrt() * 2.9 + 2.1
            } else if expected_hive_count > 1.0 {
                0.3 + 0.2 * expected_hive_count
            } else {
                0.5
            };
            let mut rand3 = self.hive_rand.borrow_mut();
            let mut initial_hive_count: i32 = -1;
            for _ in 0..65 {
                let r1_2 = rand3.next_f64();
                let r2_2 = rand3.next_f64();
                initial_hive_count =
                    (rand_normal(expected_hive_count, hive_std_dev, r1_2, r2_2) + 0.5) as i32;
                if initial_hive_count >= 0 && initial_hive_count <= max_hive_count {
                    break;
                }
            }
            initial_hive_count = initial_hive_count.clamp(0, max_hive_count);

            if self.star_type == StarType::BlackHole {
                (((self.game_desc.hive_max_density * 1000.0
                    + (self.max_hive_count_modifier as f64)
                    + 0.5) as i32)
                    / 1000)
                    .max(initial_hive_count)
                    .max(1)
            } else {
                initial_hive_count
            }
        })
    }
}

impl Serialize for Star<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Star", 14)?;
        state.serialize_field("index", &self.index)?;
        state.serialize_field("position", &self.position)?;
        state.serialize_field("mass", &self.get_mass())?;
        state.serialize_field("lifetime", &self.get_lifetime())?;
        state.serialize_field("age", &self.get_age())?;
        state.serialize_field("temperature", &self.get_temperature())?;
        state.serialize_field("type", &self.star_type)?;
        state.serialize_field("spectr", &self.get_spectr())?;
        state.serialize_field("luminosity", &self.get_luminosity())?;
        state.serialize_field("radius", &self.get_radius())?;
        state.serialize_field("dysonRadius", &self.get_dyson_radius())?;
        state.serialize_field("initialHiveCount", &self.get_initial_hive_count())?;
        state.serialize_field("maxHiveCount", &self.get_max_hive_count())?;
        state.serialize_field("color", &self.get_color())?;
        state.end()
    }
}

fn rand_normal(average_value: f32, standard_deviation: f32, r1: f64, r2: f64) -> f32 {
    average_value
        + standard_deviation * ((-2.0 * (1.0 - r1).ln()).sqrt() * (2.0 * PI * r2).sin()) as f32
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}
