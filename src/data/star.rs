use super::enums::{SpectrType, StarType, VeinType};
use super::game_desc::GameDesc;
use super::random::DspRandom;
use super::vector3::Vector3;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Star<'a> {
    pub game_desc: &'a GameDesc,
    pub used_theme_ids: RefCell<Vec<i32>>,
    pub index: i32,
    pub seed: i32,
    pub name_seed: i32,
    pub position: Vector3,
    pub name: String,
    pub level: f32,
    pub star_type: StarType,
    pub vein_patch: HashMap<VeinType, f32>,
    pub vein_amount: HashMap<VeinType, f32>,
    age_factor: f64,
    age_num1: f32,
    age_num2: f32,
    age_num3: f32,
    lifetime_factor: f64,
    radius_factor: f64,
    unmodified_mass: f32,
    pub planets_seed: i32,
}

impl<'a> Star<'a> {
    pub fn new(
        game_desc: &'a GameDesc,
        index: i32,
        seed: i32,
        position: Vector3,
        need_type: StarType,
        need_spectr: &SpectrType,
    ) -> Self {
        let mut star = Self {
            game_desc,
            used_theme_ids: RefCell::new(vec![]),
            index,
            seed,
            name_seed: 0,
            position,
            name: Default::default(),
            level: (index as f32) / ((game_desc.star_count - 1) as f32),
            star_type: need_type,
            vein_patch: HashMap::new(),
            vein_amount: HashMap::new(),
            age_factor: 0.0,
            age_num1: 0.0,
            age_num2: 0.0,
            age_num3: 0.0,
            lifetime_factor: 0.0,
            radius_factor: 0.0,
            unmodified_mass: 0.0,
            planets_seed: 0,
        };
        let mut rand1 = DspRandom::new(seed);
        star.name_seed = rand1.next_seed();
        let mut rand2 = DspRandom::new(rand1.next_seed());
        rand1.next_f64();
        star.planets_seed = rand1.next_seed();
        let r1_1 = rand2.next_f64();
        let r2_1 = rand2.next_f64();
        star.age_factor = rand2.next_f64();
        let rn = rand2.next_f64();
        let rt = rand2.next_f64();
        star.age_num1 = (rn * 0.1 + 0.95) as f32;
        star.age_num2 = (rt * 0.4 + 0.8) as f32;
        star.age_num3 = (rt * 9.0 + 1.0) as f32;
        let mass_factor = if star.is_birth() {
            0.0
        } else {
            rand2.next_f64()
        };
        star.lifetime_factor = rand2.next_f64();
        let y = rand2.next_f64() * 0.4 - 0.2;
        star.radius_factor = 2_f64.powf(y);

        star.unmodified_mass = if star.is_birth() {
            let p1 = rand_normal(0.0, 0.08, r1_1, r2_1).clamp(-0.2, 0.2);
            2_f32.powf(p1)
        } else {
            match star.star_type {
                StarType::WhiteDwarf => (1.0 + r2_1 * 5.0) as f32,
                StarType::NeutronStar => (7.0 + r1_1 * 11.0) as f32,
                StarType::BlackHole => (18.0 + r1_1 * r2_1 * 30.0) as f32,
                _ => {
                    let num8 = match need_spectr {
                        SpectrType::M => -3_f32,
                        SpectrType::O => 4.65_f32,
                        _ => {
                            let num7 = -0.98 + (0.88 + 0.98) * star.level.clamp(0.0, 1.0);
                            let average_value = if star.star_type == StarType::GiantStar {
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
                            let standard_deviation = if star.star_type == StarType::GiantStar {
                                0.3_f32
                            } else {
                                0.33_f32
                            };
                            let num = rand_normal(average_value, standard_deviation, r1_1, r2_1);
                            (if num <= 0.0 { num } else { num * 2.0 }).clamp(-2.4, 4.65)
                        }
                    };
                    2_f32.powf((num8 as f64 + (mass_factor - 0.5) * 0.2 + 1.0) as f32)
                }
            }
        };
        star
    }

    pub fn is_birth(&self) -> bool {
        return self.index == 0;
    }

    pub fn get_resource_coef(&self) -> f32 {
        if self.is_birth() {
            0.6
        } else {
            let mut num1 = (self.position.magnitude() as f32) / 32.0;
            if (num1 as f64) > 1.0 {
                num1 = ((((num1.ln() + 1.0).ln() + 1.0).ln() + 1.0).ln() + 1.0).ln() + 1.0
            }
            7.0_f32.powf(num1) * 0.6
        }
    }

    pub fn get_lifetime(&self) -> f32 {
        let d = if self.unmodified_mass < 2.0 {
            2.0 + 0.4 * (1.0 - (self.unmodified_mass as f64))
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
            * 0.1_f64.powf(((self.unmodified_mass as f64) * mass_multiplier).log(d) + 1.0)
            * (self.lifetime_factor * 0.2 + 0.9))
            + lifetime_delta;

        if self.is_birth() {
            lifetime as f32
        } else {
            let age = self.get_age();
            let mut num9 = (lifetime as f32) * age;
            if num9 > 5000.0 {
                num9 = (((num9 / 5000.0).ln() as f64 + 1.0) * 5000.0) as f32;
            }
            if num9 > 8000.0 {
                num9 =
                    (((((num9 / 8000.0).ln() + 1.0).ln() + 1.0).ln() as f64 + 1.0) * 8000.0) as f32;
            }
            num9 / age
        }
    }

    pub fn get_age(&self) -> f32 {
        (if self.is_birth() {
            self.age_factor * 0.4 + 0.3
        } else {
            match self.star_type {
                StarType::GiantStar => self.age_factor * 0.04 + 0.96,
                StarType::WhiteDwarf | StarType::NeutronStar | StarType::BlackHole => {
                    self.age_factor * 0.4 + 1.0
                }
                _ => {
                    if self.unmodified_mass >= 0.8 {
                        self.age_factor * 0.7 + 0.2
                    } else if self.unmodified_mass >= 0.5 {
                        self.age_factor * 0.4 + 0.1
                    } else {
                        self.age_factor * 0.12 + 0.02
                    }
                }
            }
        }) as f32
    }

    pub fn get_temperature_factor(&self) -> f32 {
        ((1.0 - (self.get_age().clamp(0.0, 1.0).powf(20.0) as f64) * 0.5) as f32)
            * self.unmodified_mass
    }

    fn get_unmodified_temperature(&self) -> f32 {
        let f1 = self.get_temperature_factor() as f64;
        (f1.powf(0.56 + 0.14 / (f1 + 4.0).log(5.0)) * 4450.0 + 1300.0) as f32
    }

    pub fn get_temperature(&self) -> f32 {
        match self.star_type {
            StarType::BlackHole => 0.0,
            StarType::NeutronStar => self.age_num3 * 1e+7,
            StarType::WhiteDwarf => self.age_num2 * 150000.0,
            _ => {
                let temperature = self.get_unmodified_temperature();
                if self.star_type == StarType::GiantStar {
                    let num5 = 1.0 - self.get_age().powf(30.0) * 0.5;
                    temperature * num5
                } else {
                    temperature
                }
            }
        }
    }

    pub fn get_class_factor(&self) -> f64 {
        let temperature = self.get_unmodified_temperature() as f64;
        let mut spectr_factor = ((temperature - 1300.0) / 4500.0).log(2.6) - 0.5;
        if spectr_factor < 0.0 {
            spectr_factor *= 4.0;
        }
        spectr_factor.clamp(-4.0, 2.0)
    }

    pub fn get_spectr(&self) -> SpectrType {
        if self.get_age() >= 1.0 {
            SpectrType::X
        } else {
            unsafe { ::std::mem::transmute((self.get_class_factor() + 4.0).round() as i32) }
        }
    }

    pub fn get_color(&self) -> f32 {
        match self.star_type {
            StarType::BlackHole | StarType::NeutronStar => 1.0,
            StarType::WhiteDwarf => 0.7,
            _ => (((self.get_class_factor() + 3.5) * 0.2) as f32).clamp(0.0, 1.0),
        }
    }

    pub fn get_luminosity(&self) -> f32 {
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
        (real.powf(0.33) * 1000.0).round() / 1000.0
    }

    pub fn get_radius(&self) -> f32 {
        if self.star_type == StarType::GiantStar {
            let mut num4 =
                (5.0_f64.powf(((self.unmodified_mass as f64).log10() - 0.7).abs()) * 5.0) as f32;
            if num4 > 10.0 {
                num4 = ((num4 * 0.1).ln() + 1.0) * 10.0;
            }
            num4 * self.age_num2
        } else {
            (((self.unmodified_mass as f64).powf(0.4) * self.radius_factor) as f32)
                * (match self.star_type {
                    StarType::NeutronStar => 0.15,
                    StarType::WhiteDwarf => 0.2,
                    _ => 1.0,
                })
        }
    }

    pub fn get_light_balance_radius(&self) -> f32 {
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
    }

    pub fn get_habitable_radius(&self) -> f32 {
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
    }

    pub fn get_mass(&self) -> f32 {
        match self.star_type {
            StarType::BlackHole => self.unmodified_mass * 2.5 * self.age_num2,
            StarType::NeutronStar | StarType::WhiteDwarf => {
                self.unmodified_mass * 0.2 * self.age_num1
            }
            StarType::GiantStar => {
                let num5 = 1.0 - self.get_age().powf(30.0) * 0.5;
                self.unmodified_mass * num5
            }
            _ => self.unmodified_mass,
        }
    }

    pub fn get_orbit_scaler(&self) -> f32 {
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
    }

    pub fn get_dyson_radius(&self) -> f32 {
        (self.get_orbit_scaler() * 0.28).max(self.get_radius() * 0.045)
    }
}

impl Serialize for Star<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Planet", 12)?;
        state.serialize_field("index", &self.index)?;
        state.serialize_field("position", &self.position)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("mass", &self.get_mass())?;
        state.serialize_field("lifetime", &self.get_lifetime())?;
        state.serialize_field("age", &self.get_age())?;
        state.serialize_field("temperature", &self.get_temperature())?;
        state.serialize_field("type", &self.star_type)?;
        state.serialize_field("spectr", &self.get_spectr())?;
        state.serialize_field("luminosity", &self.get_luminosity())?;
        state.serialize_field("radius", &self.get_radius())?;
        state.serialize_field("dysonRadius", &self.get_dyson_radius())?;
        state.end()
    }
}

const PI: f64 = 3.14159265358979;
fn rand_normal(average_value: f32, standard_deviation: f32, r1: f64, r2: f64) -> f32 {
    average_value
        + standard_deviation * ((-2.0 * (1.0 - r1).ln()).sqrt() * (2.0 * PI * r2).sin()) as f32
}
