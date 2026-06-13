use crate::data::birth_points::gen_birth_points;
use crate::data::planet_algorithms::create_and_prepare_algo;
use crate::data::planet_algorithms::PlanetAlgorithm;
use crate::data::planet_raw_data::query_height;
use crate::data::vector_f2::VectorF2;

use super::enums::{PlanetType, SpectrType, StarType, ThemeDistribute, VeinType};
use super::pose::Pose;
use super::quaternion::Quaternion;
use super::random::DspRandom;
use super::star::Star;
use super::theme_proto::{ThemeProto, THEME_PROTOS};
use super::vector_f3::VectorF3;
use super::vein::{ActualVein, EstimatedVein};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::cell::{OnceCell, RefCell};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::rc::Rc;
use std::vec;

#[derive(Debug)]
pub struct Planet<'a> {
    pub star: Rc<Star<'a>>,
    pub index: usize,
    pub seed: i32,
    pub theme_seed: i32,
    pub orbit_around: RefCell<Option<&'a Planet<'a>>>,
    pub orbit_index: usize,
    pub radius: f32,
    pub scale: f32,
    obliquity_scale: f64,
    rotation_param: f64,
    rotation_scale: f64,
    orbit_inclination_factor: f64,
    orbit_radius_factor: f64,
    habitable_factor: f64,
    type_factor: f64,
    gas_giant: bool,
    pub orbit_longitude: f32,
    pub orbit_phase: f32,
    pub rotation_phase: f32,
    theme_rand1: f64,
    theme_rand2: f64,
    theme_rand3: f64,
    theme_rand4: f64,
    orbital_radius: OnceCell<f32>,
    sun_distance: OnceCell<f32>,
    temperature_factor: OnceCell<f32>,
    orbital_period: OnceCell<f64>,
    obliquity: OnceCell<f32>,
    eligible_for_resonance: OnceCell<bool>,
    rotation_period: OnceCell<f64>,
    theme: OnceCell<&'static ThemeProto>,
    gases: OnceCell<Vec<(i32, f32)>>,
    estimated_veins: OnceCell<Vec<EstimatedVein>>,
    actual_veins: OnceCell<Vec<ActualVein>>,
    theme_algo_id: OnceCell<i32>,
    theme_mod_x: OnceCell<f64>,
    theme_mod_y: OnceCell<f64>,
}

const ORBIT_RADIUS: &'static [f32] = &[
    0.0, 0.4, 0.7, 1.0, 1.4, 1.9, 2.5, 3.3, 4.3, 5.5, 6.9, 8.4, 10.0, 11.7, 13.5, 15.4, 17.5,
];

impl<'a> Planet<'a> {
    pub fn new(
        star: Rc<Star<'a>>,
        index: usize,
        orbit_index: usize,
        gas_giant: bool,
        info_seed: i32,
        gen_seed: i32,
    ) -> Self {
        let mut rand = DspRandom::new(info_seed);

        let num3 = rand.next_f64();
        let num4 = rand.next_f64();
        let orbit_radius_factor = num3 * (num4 - 0.5) * 0.5;
        let orbit_inclination_factor = rand.next_f64();
        let orbit_longitude = (rand.next_f64() * 360.0) as f32;
        let orbit_phase = (rand.next_f64() * 360.0) as f32;
        let num8 = rand.next_f64();
        let num9 = rand.next_f64();
        let obliquity_scale = num8 * (num9 - 0.5);
        let num10 = rand.next_f64();
        let num11 = rand.next_f64();
        let rotation_scale = num10 * num11 * 1000.0 + 400.0;
        let rotation_phase = (rand.next_f64() * 360.0) as f32;
        let habitable_factor = rand.next_f64();
        let type_factor = rand.next_f64();
        let theme_rand1 = rand.next_f64();
        let rotation_param = rand.next_f64();
        let theme_rand2 = rand.next_f64();
        let theme_rand3 = rand.next_f64();
        let theme_rand4 = rand.next_f64();
        let theme_seed = rand.next_seed();

        let (radius, scale) = if gas_giant {
            (80.0, 10.0)
        } else {
            (200.0, 1.0)
        };

        Self {
            star,
            index,
            seed: gen_seed,
            theme_seed,
            orbit_around: RefCell::new(None),
            orbit_index,
            radius,
            scale,
            orbit_longitude,
            orbit_phase,
            rotation_phase,
            theme_rand1,
            theme_rand2,
            theme_rand3,
            theme_rand4,
            obliquity_scale,
            rotation_param,
            rotation_scale,
            orbit_inclination_factor,
            orbit_radius_factor,
            habitable_factor,
            type_factor,
            gas_giant,
            orbital_radius: OnceCell::new(),
            sun_distance: OnceCell::new(),
            temperature_factor: OnceCell::new(),
            orbital_period: OnceCell::new(),
            obliquity: OnceCell::new(),
            eligible_for_resonance: OnceCell::new(),
            rotation_period: OnceCell::new(),
            theme: OnceCell::new(),
            gases: OnceCell::new(),
            estimated_veins: OnceCell::new(),
            theme_algo_id: OnceCell::new(),
            theme_mod_x: OnceCell::new(),
            theme_mod_y: OnceCell::new(),
            actual_veins: OnceCell::new(),
        }
    }

    pub fn real_radius(&self) -> f32 {
        self.radius * self.scale
    }

    pub fn is_gas_giant(&self) -> bool {
        self.gas_giant
    }

    pub fn is_birth(&self) -> bool {
        self.orbit_index == 1 && self.star.is_birth() && self.has_orbit_around()
    }

    pub fn has_orbit_around(&self) -> bool {
        self.orbit_around.borrow().is_some()
    }

    pub fn get_orbital_radius(&self) -> f32 {
        *self.orbital_radius.get_or_init(|| {
            let a = 1.2_f32.powf(self.orbit_radius_factor as f32);
            if let Some(orbit_planet) = self.orbit_around.borrow().as_deref() {
                (((1600.0 * (self.orbit_index as f64) + 200.0)
                    * (self.star.get_orbit_scaler().powf(0.3) as f64)
                    * ((a + (1.0 - a) * 0.5) as f64)
                    + (orbit_planet.real_radius() as f64))
                    / 40000.0) as f32
            } else {
                let b = ORBIT_RADIUS[self.orbit_index] * self.star.get_orbit_scaler();
                let num16 = (((a - 1.0) as f64) / (b.max(1.0) as f64) + 1.0) as f32;
                b * num16
            }
        })
    }

    pub fn get_sun_distance(&self) -> f32 {
        *self.sun_distance.get_or_init(|| {
            if let Some(orbit_planet) = self.orbit_around.borrow().as_deref() {
                orbit_planet.get_orbital_radius()
            } else {
                self.get_orbital_radius()
            }
        })
    }

    pub fn get_temperature_factor(&self) -> f32 {
        *self.temperature_factor.get_or_init(|| {
            if self.is_gas_giant() {
                0.0
            } else {
                let habitable_radius = self.star.get_habitable_radius();
                if habitable_radius > 0.0 {
                    self.get_sun_distance() / habitable_radius
                } else {
                    1000.0
                }
            }
        })
    }

    fn get_habitable_bias(&self) -> f32 {
        if self.is_gas_giant() {
            1000.0
        } else {
            let habitable_radius = self.star.get_habitable_radius();
            let num21 = if habitable_radius > 0.0 {
                (self.get_sun_distance() / habitable_radius).ln().abs()
            } else {
                1000.0
            };
            let num22 = habitable_radius.sqrt().clamp(1.0, 2.0) - 0.04;
            num21 * num22
        }
    }

    fn get_temperature_bias(&self) -> f32 {
        if self.is_gas_giant() {
            0.0
        } else {
            let f2 = self.get_temperature_factor();
            (1.2 / ((f2 as f64) + 0.2) - 1.0) as f32
        }
    }

    fn get_luminosity(&self) -> f32 {
        let mut luminosity =
            (self.star.get_light_balance_radius() / (self.get_sun_distance() + 0.01)).powf(0.6);
        if luminosity > 1.0 {
            luminosity = luminosity.ln() + 1.0;
            luminosity = luminosity.ln() + 1.0;
            luminosity = luminosity.ln() + 1.0;
        }
        (luminosity * 100.0).round_ties_even() / 100.0
    }

    fn increment_habitable_count(&self) {
        self.star
            .game_desc
            .habitable_count
            .set(self.star.game_desc.habitable_count.get() + 1);
    }

    fn get_unmodified_planet_type(&self) -> PlanetType {
        if self.is_gas_giant() {
            PlanetType::Gas
        } else if self.is_birth() {
            self.increment_habitable_count();
            PlanetType::Ocean
        } else {
            let f2 = self.get_temperature_factor();
            if !self.star.is_birth() {
                let star_count = self.star.game_desc.star_count;
                let num18 = ((star_count as f32) * 0.29).ceil().max(11.0);
                let num19 = (num18 as f64) - (self.star.game_desc.habitable_count.get() as f64);
                let num20 = (star_count - self.star.index) as f32;
                let num23 = num20 as f64;
                let a = (num19 / num23) as f32;
                let num24 = (a + (0.35 - a) * 0.5).clamp(0.08, 0.8);
                let num25 = (self.get_habitable_bias() / num24)
                    .clamp(0.0, 1.1)
                    .powf(num24 * 10.0);
                if self.habitable_factor > (num25 as f64) {
                    self.increment_habitable_count();
                    return PlanetType::Ocean;
                }
            }
            if f2 < 5.0 / 6.0 {
                let num26 = ((f2 as f64) * 2.5 - 0.85).max(0.15);
                if self.type_factor >= num26 {
                    PlanetType::Volcano
                } else {
                    PlanetType::Desert
                }
            } else if f2 < 1.2 {
                PlanetType::Desert
            } else {
                let num27 = 0.9 / (f2 as f64) - 0.1;
                if self.type_factor >= num27 {
                    PlanetType::Ice
                } else {
                    PlanetType::Desert
                }
            }
        }
    }

    pub fn is_tidal_locked(&self) -> bool {
        self.get_rotation_period() == self.get_orbital_period()
    }

    fn get_orbit_inclination(&self) -> f32 {
        let mut orbit_inclination = (self.orbit_inclination_factor * 16.0 - 8.0) as f32;
        if self.has_orbit_around() {
            orbit_inclination *= 2.2;
        }
        if self.star.star_type == StarType::NeutronStar {
            if orbit_inclination > 0.0 {
                orbit_inclination += 3.0;
            } else {
                orbit_inclination -= 3.0;
            }
        }
        orbit_inclination
    }

    fn get_sun_orbital_period(&self) -> f64 {
        if let Some(orbit_planet) = self.orbit_around.borrow().as_deref() {
            orbit_planet.get_orbital_period()
        } else {
            self.get_orbital_period()
        }
    }

    pub fn get_orbital_period(&self) -> f64 {
        *self.orbital_period.get_or_init(|| {
            const FOUR_PI_SQUARE: f64 = 4.0 * PI * PI;
            let f1 = self.get_orbital_radius() as f64;
            (FOUR_PI_SQUARE * f1 * f1 * f1
                / (if self.has_orbit_around() {
                    1.08308421068537e-08
                } else {
                    1.35385519905204e-06 * (self.star.get_mass() as f64)
                }))
            .sqrt()
        })
    }

    pub fn get_obliquity(&self) -> f32 {
        *self.obliquity.get_or_init(|| {
            let mut obliquity: f32;
            if self.rotation_param < 0.04 {
                obliquity = (self.obliquity_scale * 39.9) as f32;
                if obliquity < 0.0 {
                    obliquity -= 70.0;
                } else {
                    obliquity += 70.0;
                }
            } else if self.rotation_param < 0.1 {
                obliquity = (self.obliquity_scale * 80.0) as f32;
                if obliquity < 0.0 {
                    obliquity -= 30.0;
                } else {
                    obliquity += 30.0;
                }
            } else {
                obliquity = (self.obliquity_scale * 60.0) as f32;
                if self.get_eligible_for_resonance() {
                    if self.rotation_param > 0.96 {
                        obliquity *= 0.01;
                    } else if self.rotation_param > 0.93 {
                        obliquity *= 0.1;
                    } else if self.rotation_param > 0.9 {
                        obliquity *= 0.2;
                    }
                }
            }
            obliquity
        })
    }

    pub fn get_eligible_for_resonance(&self) -> bool {
        *self.eligible_for_resonance.get_or_init(|| {
            let gas_giant = self.is_gas_giant();
            !self.has_orbit_around() && self.orbit_index <= 4 && !gas_giant
        })
    }

    pub fn get_rotation_period(&self) -> f64 {
        *self.rotation_period.get_or_init(|| {
            if self.get_eligible_for_resonance() {
                if self.rotation_param > 0.96 {
                    return self.get_orbital_period();
                } else if self.rotation_param > 0.93 {
                    return self.get_orbital_period() * 0.5;
                } else if self.rotation_param > 0.9 {
                    return self.get_orbital_period() * 0.25;
                }
            }
            let gas_giant = self.is_gas_giant();
            let mut rotation_period = self.rotation_scale
                * (if gas_giant {
                    0.2
                } else {
                    match self.star.star_type {
                        StarType::WhiteDwarf => 0.5,
                        StarType::NeutronStar => 0.2,
                        StarType::BlackHole => 0.15,
                        _ => 1.0,
                    }
                })
                * (if self.has_orbit_around() {
                    1.0
                } else {
                    self.get_orbital_radius().powf(0.25) as f64
                });
            rotation_period = 1.0 / (1.0 / self.get_sun_orbital_period() + 1.0 / rotation_period);
            if self.rotation_param > 0.85 && self.rotation_param <= 0.9 {
                rotation_period = -rotation_period;
            }
            rotation_period
        })
    }

    pub fn get_theme(&self) -> &'static ThemeProto {
        self.theme.get_or_init(|| {
            let mut potential_themes: Vec<&'static ThemeProto> = Vec::new();
            let mut used_theme_ids = self.star.used_theme_ids.borrow_mut();
            let unused_themes: Vec<&'static ThemeProto> = THEME_PROTOS
                .iter()
                .filter(|&theme| !used_theme_ids.contains(&theme.id))
                .collect();
            let planet_type = self.get_unmodified_planet_type();
            let temperature_bias = self.get_temperature_bias();
            for theme in &unused_themes {
                if self.star.is_birth() && planet_type == PlanetType::Ocean {
                    if theme.distribute == ThemeDistribute::Birth {
                        potential_themes.push(theme);
                    }
                } else {
                    let flag2 = if theme.temperature.abs() < 0.5
                        && theme.planet_type == PlanetType::Desert
                    {
                        (temperature_bias.abs() as f64) < (theme.temperature.abs() as f64) + 0.1
                    } else {
                        (theme.temperature as f64) * (temperature_bias as f64) >= -0.1
                    };
                    if (theme.planet_type == planet_type) && flag2 {
                        if self.star.is_birth() {
                            if theme.distribute == ThemeDistribute::Default {
                                potential_themes.push(theme);
                            }
                        } else if theme.distribute == ThemeDistribute::Default
                            || theme.distribute == ThemeDistribute::Interstellar
                        {
                            potential_themes.push(theme);
                        }
                    }
                }
            }
            if potential_themes.is_empty() {
                for theme in &unused_themes {
                    if theme.planet_type == PlanetType::Desert {
                        potential_themes.push(theme);
                    }
                }
            }
            if potential_themes.is_empty() {
                for theme in &*THEME_PROTOS {
                    if theme.planet_type == PlanetType::Desert {
                        potential_themes.push(theme);
                    }
                }
            }
            let theme_proto = potential_themes[((self.theme_rand1 * (potential_themes.len() as f64))
                as usize)
                % potential_themes.len()];
            used_theme_ids.push(theme_proto.id);
            theme_proto
        })
    }

    pub fn get_algo_id(&self) -> i32 {
        *self.theme_algo_id.get_or_init(|| {
            let theme = self.get_theme();
            if theme.algos.is_empty() {
                0
            } else {
                *theme
                    .algos
                    .get(
                        (self.theme_rand2 * (theme.algos.len() as f64)) as usize
                            % theme.algos.len(),
                    )
                    .unwrap()
            }
        })
    }

    pub fn get_mod_x(&self) -> f64 {
        *self.theme_mod_x.get_or_init(|| {
            let theme = self.get_theme();
            if theme.algos.is_empty() {
                0.0
            } else {
                theme.mod_x.0 + self.theme_rand3 * (theme.mod_x.1 - theme.mod_x.0)
            }
        })
    }

    pub fn get_mod_y(&self) -> f64 {
        *self.theme_mod_y.get_or_init(|| {
            let theme = self.get_theme();
            if theme.algos.is_empty() {
                0.0
            } else {
                theme.mod_y.0 + self.theme_rand4 * (theme.mod_y.1 - theme.mod_y.0)
            }
        })
    }

    pub fn get_type(&self) -> &PlanetType {
        &self.get_theme().planet_type
    }
    pub fn get_gases(&self) -> &Vec<(i32, f32)> {
        self.gases.get_or_init(|| {
            let mut gases: Vec<(i32, f32)> = vec![];
            if !self.is_gas_giant() {
                return gases;
            }
            let gas_coef = self.star.game_desc.gas_coef();
            let mut rand = DspRandom::new(self.theme_seed);
            let theme_proto = self.get_theme();
            let coef = self.star.get_resource_coef().powf(0.3);
            for (item, speed) in theme_proto
                .gas_items
                .iter()
                .zip(theme_proto.gas_speeds.iter())
            {
                let num2 = speed * (rand.next_f32() * 21.0 / 110.0 + 10.0 / 11.0) * gas_coef;
                gases.push((*item, num2 * coef))
            }
            gases
        })
    }

    pub fn can_have_vein(&self, vein_type: &VeinType) -> bool {
        let theme = self.get_theme();
        if vein_type.is_rare() {
            theme.rare_veins.contains(vein_type)
        } else {
            let vein_index = *vein_type as i32;
            if let Some(x) = theme.vein_spot.get((vein_index - 1) as usize) {
                *x != 0
            } else {
                false
            }
        }
    }

    pub fn get_estimated_veins(&self) -> &Vec<EstimatedVein> {
        self.estimated_veins.get_or_init(|| {
            let mut output: Vec<EstimatedVein> = vec![];
            if self.is_gas_giant() {
                return output;
            }
            let mut rand1 = DspRandom::new(self.seed);
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            let theme_proto = self.get_theme();
            let mut num_array_1: Vec<i32> = (0..15_i32)
                .map(|i| *theme_proto.vein_spot.get((i - 1) as usize).unwrap_or(&0))
                .collect();
            let mut num_array_2: Vec<f32> = (0..15_i32)
                .map(|i| *theme_proto.vein_count.get((i - 1) as usize).unwrap_or(&0.0))
                .collect();
            let mut num_array_3: Vec<f32> = (0..15_i32)
                .map(|i| {
                    *theme_proto
                        .vein_opacity
                        .get((i - 1) as usize)
                        .unwrap_or(&0.0)
                })
                .collect();
            let mut add_until = |i: &mut i32, t: f64| {
                for _ in 1..12 {
                    if rand1.next_f64() >= t {
                        break;
                    }
                    *i += 1;
                }
            };
            let p: f32 = match self.star.star_type {
                StarType::MainSeqStar => match self.star.get_spectr() {
                    SpectrType::M => 2.5,
                    SpectrType::G => 0.7,
                    SpectrType::F => 0.6,
                    SpectrType::B => 0.4,
                    SpectrType::O => 1.6,
                    _ => 1.0,
                },
                StarType::GiantStar => 2.5,
                StarType::WhiteDwarf => {
                    num_array_1[9] += 2;
                    add_until(num_array_1.get_mut(9).unwrap(), 0.45);
                    num_array_2[9] = 0.7;
                    num_array_3[9] = 1.0;
                    num_array_1[10] += 2;
                    add_until(num_array_1.get_mut(10).unwrap(), 0.45);
                    num_array_2[10] = 0.7;
                    num_array_3[10] = 1.0;
                    num_array_1[12] += 1;
                    add_until(num_array_1.get_mut(12).unwrap(), 0.5);
                    num_array_2[12] = 0.7;
                    num_array_3[12] = 0.3;
                    3.5
                }
                StarType::NeutronStar => {
                    num_array_1[14] += 1;
                    add_until(num_array_1.get_mut(14).unwrap(), 0.65);
                    num_array_2[14] = 0.7;
                    num_array_3[14] = 0.3;
                    4.5
                }
                StarType::BlackHole => {
                    num_array_1[14] += 1;
                    add_until(num_array_1.get_mut(14).unwrap(), 0.65);
                    num_array_2[14] = 0.7;
                    num_array_3[14] = 0.3;
                    5.0
                }
            };
            let is_rare_resource = self.star.game_desc.is_rare_resource();
            let mut f = self.star.get_resource_coef();
            if theme_proto.distribute == ThemeDistribute::Birth {
                f *= 2.0 / 3.0;
            } else if is_rare_resource {
                if f > 1.0 {
                    f = f.powf(0.8)
                }
                f *= 0.7;
            }
            for (index1, rare_vein_ref) in theme_proto.rare_veins.iter().enumerate() {
                let rare_vein = rare_vein_ref.clone() as usize;
                let num2 = theme_proto.rare_settings
                    [index1 * 4 + (if self.star.is_birth() { 0 } else { 1 })];
                let rare_setting_1 = theme_proto.rare_settings[index1 * 4 + 2];
                let rare_setting_2 = theme_proto.rare_settings[index1 * 4 + 3];
                let num4 = 1.0 - (1.0 - num2).powf(p);
                let num5 = 1.0 - (1.0 - rare_setting_2).powf(p);
                if rand1.next_f64() < (num4 as f64) {
                    num_array_1[rare_vein] += 1;
                    num_array_2[rare_vein] = num5;
                    num_array_3[rare_vein] = num5;
                    for _ in 1..12 {
                        if rand1.next_f64() >= (rare_setting_1 as f64) {
                            break;
                        }
                        num_array_1[rare_vein] += 1;
                    }
                }
            }
            let is_infinite_resource = self.star.game_desc.is_infinite_resource();
            for index3 in 1..15 {
                let num8 = num_array_1[index3 as usize];
                if num8 > 0 {
                    let vein_type: VeinType = unsafe { ::std::mem::transmute(index3) };
                    let mut vein = EstimatedVein::new();
                    vein.vein_type = vein_type;
                    vein.min_group = num8 - 1;
                    vein.max_group = num8 + 1;
                    if vein.vein_type == VeinType::Oil {
                        vein.min_patch = 1;
                        vein.max_patch = 1;
                    } else {
                        let num12 = num_array_2[index3 as usize];
                        vein.min_patch = (num12 * 20.0).round_ties_even() as i32;
                        vein.max_patch = (num12 * 24.0).round_ties_even() as i32;
                    }
                    let num16 = if vein.vein_type == VeinType::Oil {
                        f.powf(0.5)
                    } else {
                        f
                    };
                    if is_infinite_resource && vein.vein_type != VeinType::Oil {
                        vein.min_amount = 1;
                        vein.max_amount = 1;
                    } else {
                        let num17 = ((num_array_3[index3 as usize] * 100000.0 * num16)
                            .round_ties_even() as i32)
                            .max(20);
                        let num18 = if num17 < 16000 {
                            ((num17 as f32) * (15.0 / 16.0)).floor() as i32
                        } else {
                            15000
                        };
                        let map_amount = |amount: i32| -> i32 {
                            let x1 = ((amount as f32) * 1.1).round_ties_even();
                            let x2 = (if vein.vein_type == VeinType::Oil {
                                x1 * self.star.game_desc.oil_amount_multiplier()
                            } else {
                                x1 * self.star.game_desc.resource_multiplier
                            })
                            .round_ties_even() as i32;
                            x2.max(1)
                        };
                        vein.min_amount = map_amount(num17 - num18);
                        vein.max_amount = map_amount(num17 + num18);
                    }
                    output.push(vein);
                }
            }
            output
        })
    }

    pub fn get_runtime_orbit_rotation(&self) -> Quaternion {
        let mut rot = Quaternion::angle_axis(self.orbit_longitude, &VectorF3::up())
            * Quaternion::angle_axis(self.get_orbit_inclination(), &VectorF3::forward());
        if let Some(parent) = self.orbit_around.borrow().as_deref() {
            rot = parent.get_runtime_orbit_rotation() * rot;
        }
        rot
    }

    pub fn get_runtime_system_rotation(&self) -> Quaternion {
        self.get_runtime_orbit_rotation()
            * Quaternion::angle_axis(self.get_obliquity(), &VectorF3::forward())
    }

    pub fn predict_pose(&self, time: f64) -> Pose {
        use std::f64::consts::PI as PI_F64;

        // Orbit angle
        let num1 = time / self.get_orbital_period() + (self.orbit_phase as f64) / 360.0;
        let num2 = (num1 + 0.1) as i32;
        let num3 = num1 - (num2 as f64);
        let num4 = num3 * 2.0 * PI_F64;

        let orbit_radius = self.get_orbital_radius() as f64;
        let local_pos = VectorF3(
            (num4.cos() * orbit_radius) as f32,
            0.0,
            (num4.sin() * orbit_radius) as f32,
        );

        let orbit_rot = self.get_runtime_orbit_rotation();
        let mut position = orbit_rot.q_rotate_lf(&local_pos);

        // If this planet orbits another planet, add the parent's position
        if let Some(parent) = self.orbit_around.borrow().as_deref() {
            let parent_pose = parent.predict_pose(time);
            position = VectorF3(
                position.0 + parent_pose.position.0,
                position.1 + parent_pose.position.1,
                position.2 + parent_pose.position.2,
            );
        }

        // Rotation angle from time
        let num5 = time / self.get_rotation_period() + (self.rotation_phase as f64) / 360.0;
        let num6 = (num5 + 0.1) as i32;
        let angle1 = (num5 - (num6 as f64)) * 360.0;

        let rotation = self.get_runtime_system_rotation()
            * Quaternion::angle_axis(angle1 as f32, &VectorF3::down());

        Pose::new(position, rotation)
    }

    /// Computes the star direction vector at time 85.0 (used in `gen_birth_points`).
    ///
    /// Port of C# lines 764-766:
    /// ```csharp
    /// Pose pose = this.PredictPose(85.0);
    /// Vector3 vector3_1 = (Vector3) Maths.QInvRotateLF(
    ///     pose.rotation, this.star.uPosition - (VectorLF3) pose.position * 40000.0);
    /// vector3_1.Normalize();
    /// ```
    ///
    /// Where `star.uPosition` = `star.position * 2400000.0`.
    pub fn get_star_direction(&self) -> VectorF3 {
        let pose = self.predict_pose(85.0);

        // star.uPosition = star.position * 2400000.0
        let star_pos = &self.star.position; // Vector3 (f64)
        let star_u_pos = VectorF3(
            (star_pos.0 * 2400000.0) as f32,
            (star_pos.1 * 2400000.0) as f32,
            (star_pos.2 * 2400000.0) as f32,
        );

        // pose.position * 40000.0
        let pose_scaled = VectorF3(
            pose.position.0 * 40000.0,
            pose.position.1 * 40000.0,
            pose.position.2 * 40000.0,
        );

        // star.uPosition - pose.position * 40000.0
        let delta = VectorF3(
            star_u_pos.0 - pose_scaled.0,
            star_u_pos.1 - pose_scaled.1,
            star_u_pos.2 - pose_scaled.2,
        );

        // QInvRotateLF(pose.rotation, delta) then normalize
        let mut dir = pose.rotation.q_inv_rotate_lf(&delta);
        dir.normalize();
        dir
    }

    fn can_place_vein(
        &self,
        vein_type: &VeinType,
        zero: &VectorF3,
        algo: &dyn PlanetAlgorithm,
    ) -> bool {
        let algo_id = self.get_algo_id();
        if algo_id == 7 && vein_type != &VeinType::Bamboo {
            return true;
        }
        let height = query_height(zero, algo);
        match algo_id {
            7 => height <= self.radius - 4.0,
            11 => {
                height >= self.radius
                    && match vein_type {
                        &VeinType::Oil => height >= self.radius + 0.5,
                        &VeinType::Iron | &VeinType::Copper => height <= self.radius + 0.7,
                        &VeinType::Silicium | &VeinType::Titanium => height > self.radius + 0.7,
                        _ => true,
                    }
            }
            12 => {
                height >= self.radius
                    && match vein_type {
                        &VeinType::Oil => height >= self.radius + 0.5,
                        &VeinType::Fireice => height >= self.radius + 1.2,
                        _ => true,
                    }
            }
            13 => {
                height >= self.radius
                    && match vein_type {
                        &VeinType::Oil => height >= self.radius + 0.5,
                        &VeinType::Iron
                        | &VeinType::Copper
                        | &VeinType::Silicium
                        | &VeinType::Titanium => height <= self.radius + 0.7,
                        _ => true,
                    }
            }
            _ => {
                height >= self.radius
                    && match vein_type {
                        &VeinType::Oil => height >= self.radius + 0.5,
                        _ => true,
                    }
            }
        }
    }

    pub fn get_actual_veins(&self) -> &Vec<ActualVein> {
        self.actual_veins.get_or_init(|| {
            if self.gas_giant {
                return vec![];
            }
            let algo = create_and_prepare_algo(self);

            let theme = self.get_theme();
            let mut rand1 = DspRandom::new(self.seed);
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            let birth_seed = rand1.next_seed();
            let mut rand2 = DspRandom::new(rand1.next_seed());
            let mut num_array_1: Vec<i32> = (0..15_i32)
                .map(|i| *theme.vein_spot.get((i - 1) as usize).unwrap_or(&0))
                .collect();
            let mut num_array_2: Vec<f32> = (0..15_i32)
                .map(|i| *theme.vein_count.get((i - 1) as usize).unwrap_or(&0.0))
                .collect();
            let mut num_array_3: Vec<f32> = (0..15_i32)
                .map(|i| *theme.vein_opacity.get((i - 1) as usize).unwrap_or(&0.0))
                .collect();

            let mut add_until = |i: &mut i32, t: f64| {
                for _ in 1..12 {
                    if rand1.next_f64() >= t {
                        break;
                    }
                    *i += 1;
                }
            };

            let star_type_multiplier: f32 = match self.star.star_type {
                StarType::MainSeqStar => match self.star.get_spectr() {
                    SpectrType::M => 2.5,
                    SpectrType::G => 0.7,
                    SpectrType::F => 0.6,
                    SpectrType::B => 0.4,
                    SpectrType::O => 1.6,
                    _ => 1.0,
                },
                StarType::GiantStar => 2.5,
                StarType::WhiteDwarf => {
                    num_array_1[9] += 2;
                    add_until(num_array_1.get_mut(9).unwrap(), 0.45);
                    num_array_2[9] = 0.7;
                    num_array_3[9] = 1.0;
                    num_array_1[10] += 2;
                    add_until(num_array_1.get_mut(10).unwrap(), 0.45);
                    num_array_2[10] = 0.7;
                    num_array_3[10] = 1.0;
                    num_array_1[12] += 1;
                    add_until(num_array_1.get_mut(12).unwrap(), 0.5);
                    num_array_2[12] = 0.7;
                    num_array_3[12] = 0.3;
                    3.5
                }
                StarType::NeutronStar => {
                    num_array_1[14] += 1;
                    add_until(num_array_1.get_mut(14).unwrap(), 0.65);
                    num_array_2[14] = 0.7;
                    num_array_3[14] = 0.3;
                    4.5
                }
                StarType::BlackHole => {
                    num_array_1[14] += 1;
                    add_until(num_array_1.get_mut(14).unwrap(), 0.65);
                    num_array_2[14] = 0.7;
                    num_array_3[14] = 0.3;
                    5.0
                }
            };

            for (index1, rare_vein_ref) in theme.rare_veins.iter().enumerate() {
                let rare_vein = rare_vein_ref.clone() as usize;
                let rare_vein_chance =
                    theme.rare_settings[index1 * 4 + (if self.star.is_birth() { 0 } else { 1 })];
                let rare_setting_1 = theme.rare_settings[index1 * 4 + 2];
                let rare_setting_2 = theme.rare_settings[index1 * 4 + 3];
                let adjusted_rare_chance =
                    1.0 - (1.0 - rare_vein_chance).powf(star_type_multiplier);
                let adjust_rare_count = 1.0 - (1.0 - rare_setting_2).powf(star_type_multiplier);
                if rand1.next_f64() < (adjusted_rare_chance as f64) {
                    num_array_1[rare_vein] += 1;
                    num_array_2[rare_vein] = adjust_rare_count;
                    num_array_3[rare_vein] = adjust_rare_count;
                    for _ in 1..12 {
                        if rand1.next_f64() >= (rare_setting_1 as f64) {
                            break;
                        }
                        num_array_1[rare_vein] += 1;
                    }
                }
            }

            let is_rare_resource = self.star.game_desc.is_rare_resource();
            let mut resource_coef = self.star.get_resource_coef();
            let is_birth_planet = theme.distribute == ThemeDistribute::Birth;
            if is_birth_planet {
                resource_coef *= 2.0 / 3.0;
            } else if is_rare_resource {
                if resource_coef > 1.0 {
                    resource_coef = resource_coef.powf(0.8)
                }
                resource_coef *= 0.7;
            }
            let mut vein_vectors: Vec<(VeinType, VectorF3, bool)> = Vec::with_capacity(512);

            let birth_point = if is_birth_planet {
                let star_direction = self.get_star_direction();
                let birth_point_data =
                    gen_birth_points(algo.as_ref(), birth_seed, self.radius, star_direction);
                vein_vectors.push((VeinType::Iron, birth_point_data.birth_resource_point0, true));
                vein_vectors.push((
                    VeinType::Copper,
                    birth_point_data.birth_resource_point1,
                    true,
                ));
                let mut birth_point = birth_point_data.birth_point;
                birth_point.normalize();
                birth_point * 0.75
            } else {
                let x = rand2.next_f64() * 2.0 - 1.0;
                let y = rand2.next_f64() - 0.5;
                let z = rand2.next_f64() * 2.0 - 1.0;
                let mut birth_point = VectorF3::new(x as f32, y as f32, z as f32);
                birth_point.normalize();
                birth_point * (rand2.next_f64() * 0.4 + 0.2) as f32
            };

            let is_infinite_resource = self.star.game_desc.is_infinite_resource();
            let mut amount_map: HashMap<&VeinType, i32> = HashMap::new();

            let min_vein_spacing = 2.1 / self.radius;
            let min_vein_spacing_sq = (min_vein_spacing as f64) * (min_vein_spacing as f64);

            for index3 in 1..15 {
                if vein_vectors.len() >= 512 {
                    break;
                }
                let mut vein_spot_count = num_array_1[index3 as usize];
                if vein_spot_count > 1 {
                    vein_spot_count += rand2.next_i32(3) - 1;
                }
                let vein_type: VeinType = unsafe { ::std::mem::transmute(index3) };
                let min_sq_dist = min_vein_spacing_sq
                    * (if vein_type == VeinType::Oil {
                        100_f64
                    } else {
                        196_f64
                    });

                for _ in 0..vein_spot_count {
                    for _ in 0..200 {
                        let x = rand2.next_f64() * 2.0 - 1.0;
                        let y = rand2.next_f64() * 2.0 - 1.0;
                        let z = rand2.next_f64() * 2.0 - 1.0;
                        let mut zero = VectorF3(x as f32, y as f32, z as f32);
                        if vein_type != VeinType::Oil {
                            zero += birth_point;
                        }
                        zero.normalize();
                        if self.can_place_vein(&vein_type, &zero, algo.as_ref()) {
                            let not_too_close_to_other_vein =
                                vein_vectors.iter().all(|(_, pos, _)| {
                                    (pos.distance_sq_from(&zero) as f64) >= min_sq_dist
                                });
                            if not_too_close_to_other_vein {
                                vein_vectors.push((vein_type, zero, false));
                                break;
                            }
                        }
                    }
                    if vein_vectors.len() >= 512 {
                        break;
                    }
                }
            }

            for (vein_type, vein_vector, is_birth_resource) in vein_vectors.iter() {
                let is_oil = vein_type == &VeinType::Oil;
                let normalized = vein_vector.normalized();
                let rotation = Quaternion::from_to_rotation(&VectorF3::up(), &normalized);
                let vector3_1 = &rotation * &VectorF3::right();
                let vector3_2 = &rotation * &VectorF3::forward();
                let index7 = *vein_type as i32;
                let target_node_count = if *is_birth_resource {
                    rand2.next_f64();
                    6
                } else if is_oil {
                    rand2.next_f64();
                    1
                } else {
                    (num_array_2.get(index7 as usize).unwrap() * (rand2.next_i32(5) + 20) as f32)
                        .round_ties_even() as usize
                };
                let mut tmp_vecs = Vec::with_capacity(target_node_count);
                tmp_vecs.push(VectorF2::zero());
                let vein_density = if *is_birth_resource {
                    0.2_f32
                } else {
                    *num_array_3.get(index7 as usize).unwrap()
                };
                for _ in 0..20 {
                    for index8 in 0..tmp_vecs.len() {
                        let vector2_1 = tmp_vecs.get(index8).unwrap();
                        if vector2_1.magnitude_sq() <= 36.0 {
                            let random_angle_radians = rand2.next_f64() * PI * 2.0;
                            let mut vector2_2 = VectorF2::new(
                                random_angle_radians.cos() as f32,
                                random_angle_radians.sin() as f32,
                            );
                            vector2_2 += vector2_1 * 0.2;
                            vector2_2.normalize();
                            let vector2_3 = vector2_1 + &vector2_2;
                            let not_too_close_to_other_node = tmp_vecs
                                .iter()
                                .all(|v| v.distance_sq_from(&vector2_3) >= 0.85);
                            if not_too_close_to_other_node {
                                tmp_vecs.push(vector2_3);
                            }
                            if tmp_vecs.len() >= target_node_count {
                                break;
                            }
                        }
                    }
                    if tmp_vecs.len() >= target_node_count {
                        break;
                    }
                }
                let adjusted_resource_coef = if is_oil {
                    resource_coef.powf(0.5)
                } else {
                    resource_coef
                };
                let total_amount = ((vein_density * 100000.0 * adjusted_resource_coef)
                    .round_ties_even() as i32)
                    .max(20);
                let amount_variance = if total_amount < 16000 {
                    ((total_amount as f32) * (15.0 / 16.0)) as i32
                } else {
                    15000
                };
                let min_value = total_amount - amount_variance;
                let value_range = amount_variance * 2 + 1;
                for pos in tmp_vecs.iter() {
                    let vector3_3 = ((vector3_1 * pos.0) + (vector3_2 * pos.1)) * min_vein_spacing;
                    let raw_amount = rand2.next_i32(value_range) + min_value;
                    let amount = if is_infinite_resource && !is_oil {
                        1
                    } else {
                        let multiplier = if is_oil {
                            self.star.game_desc.oil_amount_multiplier()
                        } else {
                            self.star.game_desc.resource_multiplier
                        };
                        (((raw_amount as f32) * 1.1 * multiplier).round_ties_even() as i32).max(1)
                    };
                    let mut pos = normalized + vector3_3;
                    if is_oil {
                        pos = self.snap_to(&pos);
                    }
                    let surface_height = query_height(&pos, algo.as_ref());
                    if theme.water_item_id == 0 || surface_height >= self.radius {
                        // println!("{:?},{:?},{}", pos * surface_height, vein_type, amount);
                        amount_map
                            .insert(vein_type, amount_map.get(vein_type).unwrap_or(&0) + amount);
                    }
                }
            }

            amount_map
                .iter()
                .map(|(vein_type, amount)| ActualVein {
                    vein_type: **vein_type,
                    amount: *amount,
                })
                .collect()
        })
    }

    pub fn is_acutal_veins_generated(&self) -> bool {
        self.actual_veins.get().is_some()
    }

    fn snap_to(&self, pos: &VectorF3) -> VectorF3 {
        let segment = ((self.radius / 4.0 + 0.1) as i32 * 4) as f32;
        let two_pi = PI as f32 * 2.0;
        let pos = pos.normalized();
        let num = pos.1.asin();
        let mut num2 = pos.0.atan2(-pos.2);
        let mut num3 = num / two_pi * segment;
        let latitude_index = (num3.abs() - 0.1).max(0.0) as i32;
        let num4 = determine_longitude_segment_count(latitude_index, segment) as f32;
        let mut num5 = num2 / two_pi * num4;
        num3 = (num3 * 5.0).round_ties_even() / 5.0;
        num5 = (num5 * 5.0).round_ties_even() / 5.0;
        let f = num3 / segment * two_pi;
        num2 = num5 / num4 * two_pi;
        let y = f.sin();
        let num6 = f.cos();
        let num7 = num2.sin();
        let num8 = num2.cos();
        VectorF3(num6 * num7, y, num6 * (-num8))
    }
}

fn determine_longitude_segment_count(latitude_index: i32, segment: f32) -> i32 {
    let num = (((latitude_index as f32) / (segment / 4.0) * PI as f32 * 0.5)
        .cos()
        .abs()
        * segment)
        .ceil() as usize;
    if num < 500 {
        SEGMENT_TABLE[num]
    } else {
        ((num as i32) + 49) / 100 * 100
    }
}

const SEGMENT_TABLE: [i32; 512] = [
    1, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 8, 16, 16, 16, 16, 20, 20, 20, 20, 20, 20, 20, 20,
    32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40,
    40, 40, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 80, 80, 80,
    80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 100, 100, 100, 100, 100, 100, 100,
    100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 120, 120, 120,
    120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
    120, 120, 120, 120, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
    160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
    160, 160, 160, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200,
    200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200,
    200, 200, 200, 200, 200, 200, 200, 200, 200, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240,
    240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240,
    240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240, 240,
    240, 240, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300,
    300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300,
    300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300,
    300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300,
    300, 300, 300, 300, 300, 300, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400,
    400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400,
    400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400,
    400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400,
    400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400,
    400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 400, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
];

impl Serialize for Planet<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Planet", 15)?;
        state.serialize_field("index", &self.index)?;
        state.serialize_field("orbitAround", &self.orbit_around.borrow().map(|p| p.index))?;
        state.serialize_field("orbitIndex", &self.orbit_index)?;
        state.serialize_field("orbitRadius", &self.get_orbital_radius())?;
        state.serialize_field("orbitInclination", &self.get_orbit_inclination())?;
        state.serialize_field("orbitLongitude", &self.orbit_longitude)?;
        state.serialize_field("orbitalPeriod", &self.get_orbital_period())?;
        state.serialize_field("obliquity", &self.get_obliquity())?;
        state.serialize_field("rotationPeriod", &self.get_rotation_period())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("luminosity", &self.get_luminosity())?;
        state.serialize_field("theme", &self.get_theme())?;
        state.serialize_field("gases", &self.get_gases())?;
        if self.star.game_desc.use_actual_veins {
            state.serialize_field("actualVeins", &self.get_actual_veins())?;
        } else {
            state.serialize_field("veins", &self.get_estimated_veins())?;
        }
        state.end()
    }
}
