use crate::data::birth_points::BirthPoints;
use crate::data::game_desc::GameDesc;
use crate::data::planet_raw_data::PlanetRawData;
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
use std::cell::{Cell, OnceCell, RefCell};
use std::f64::consts::PI;
use std::rc::Rc;

#[derive(Debug)]
pub struct Planet<'a> {
    game_desc: &'a GameDesc,
    pub star: Rc<Star<'a>>,
    pub index: usize,
    habitable_count: &'a Cell<i32>,
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
}

const ORBIT_RADIUS: &'static [f32] = &[
    0.0, 0.4, 0.7, 1.0, 1.4, 1.9, 2.5, 3.3, 4.3, 5.5, 6.9, 8.4, 10.0, 11.7, 13.5, 15.4, 17.5,
];

impl<'a> Planet<'a> {
    pub fn new(
        game_desc: &'a GameDesc,
        star: Rc<Star<'a>>,
        index: usize,
        habitable_count: &'a Cell<i32>,
        orbit_index: usize,
        gas_giant: bool,
        info_seed: i32,
        gen_seed: i32,
    ) -> Self {
        let mut rand = DspRandom::new(info_seed);

        let orbit_radius_rand1 = rand.next_f64();
        let orbit_radius_rand2 = rand.next_f64();
        let orbit_radius_factor = orbit_radius_rand1 * (orbit_radius_rand2 - 0.5) * 0.5;
        let orbit_inclination_factor = rand.next_f64();
        let orbit_longitude = (rand.next_f64() * 360.0) as f32;
        let orbit_phase = (rand.next_f64() * 360.0) as f32;
        let obliquity_rand1 = rand.next_f64();
        let obliquity_rand2 = rand.next_f64();
        let obliquity_scale = obliquity_rand1 * (obliquity_rand2 - 0.5);
        let rotation_rand1 = rand.next_f64();
        let rotation_rand2 = rand.next_f64();
        let rotation_scale = rotation_rand1 * rotation_rand2 * 1000.0 + 400.0;
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
            game_desc,
            star,
            index,
            habitable_count,
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
                let adjusted_orbit_radius = (((a - 1.0) as f64) / (b.max(1.0) as f64) + 1.0) as f32;
                b * adjusted_orbit_radius
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
            let distance_log_factor = if habitable_radius > 0.0 {
                (self.get_sun_distance() / habitable_radius).ln().abs()
            } else {
                1000.0
            };
            let habitable_radius_sqrt_clamped = habitable_radius.sqrt().clamp(1.0, 2.0) - 0.04;
            distance_log_factor * habitable_radius_sqrt_clamped
        }
    }

    fn get_temperature_bias(&self) -> f32 {
        if self.is_gas_giant() {
            0.0
        } else {
            let temperature_factor_val = self.get_temperature_factor();
            (1.2 / ((temperature_factor_val as f64) + 0.2) - 1.0) as f32
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
        self.habitable_count.set(self.habitable_count.get() + 1);
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
                let star_count = self.game_desc.star_count;
                let habitable_ceiling = ((star_count as f32) * 0.29).ceil().max(11.0);
                let remaining_habitable_slots =
                    (habitable_ceiling as f64) - (self.habitable_count.get() as f64);
                let remaining_stars = (star_count - self.star.index) as f32;
                let remaining_stars_f64 = remaining_stars as f64;
                let remaining_ratio = (remaining_habitable_slots / remaining_stars_f64) as f32;
                let allocation_probability =
                    (remaining_ratio + (0.35 - remaining_ratio) * 0.5).clamp(0.08, 0.8);
                let habitable_bias_threshold = (self.get_habitable_bias() / allocation_probability)
                    .clamp(0.0, 1.1)
                    .powf(allocation_probability * 10.0);
                if self.habitable_factor > (habitable_bias_threshold as f64) {
                    self.increment_habitable_count();
                    return PlanetType::Ocean;
                }
            }
            if f2 < 5.0 / 6.0 {
                let volcano_type_threshold = ((f2 as f64) * 2.5 - 0.85).max(0.15);
                if self.type_factor >= volcano_type_threshold {
                    PlanetType::Volcano
                } else {
                    PlanetType::Desert
                }
            } else if f2 < 1.2 {
                PlanetType::Desert
            } else {
                let ice_type_threshold = 0.9 / (f2 as f64) - 0.1;
                if self.type_factor >= ice_type_threshold {
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
            let orbital_radius_f64 = self.get_orbital_radius() as f64;
            (FOUR_PI_SQUARE * orbital_radius_f64 * orbital_radius_f64 * orbital_radius_f64
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
                    let temperature_matches = if theme.temperature.abs() < 0.5
                        && theme.planet_type == PlanetType::Desert
                    {
                        (temperature_bias.abs() as f64) < (theme.temperature.abs() as f64) + 0.1
                    } else {
                        (theme.temperature as f64) * (temperature_bias as f64) >= -0.1
                    };
                    if (theme.planet_type == planet_type) && temperature_matches {
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
        let theme = self.get_theme();
        if theme.algos.is_empty() {
            0.0
        } else {
            theme.mod_x.0 + self.theme_rand3 * (theme.mod_x.1 - theme.mod_x.0)
        }
    }

    pub fn get_mod_y(&self) -> f64 {
        let theme = self.get_theme();
        if theme.algos.is_empty() {
            0.0
        } else {
            theme.mod_y.0 + self.theme_rand4 * (theme.mod_y.1 - theme.mod_y.0)
        }
    }

    pub fn get_type(&self) -> &PlanetType {
        &self.get_theme().planet_type
    }
    pub fn get_gases(&self) -> &Vec<(i32, f32)> {
        self.gases.get_or_init(|| {
            if !self.is_gas_giant() {
                return Vec::with_capacity(0);
            }
            let mut gases: Vec<(i32, f32)> = Vec::with_capacity(2);
            let gas_coef = self.game_desc.gas_coef();
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
        } else if vein_type == &VeinType::Mag {
            matches!(
                self.star.star_type,
                StarType::BlackHole | StarType::NeutronStar
            )
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
            if self.is_gas_giant() {
                return Vec::with_capacity(0);
            }
            let mut output: Vec<EstimatedVein> = Vec::with_capacity(14);
            let mut rand1 = DspRandom::new(self.seed);
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            let theme_proto = self.get_theme();
            let mut vein_spots: Vec<i32> = (0..15_i32)
                .map(|i| *theme_proto.vein_spot.get((i - 1) as usize).unwrap_or(&0))
                .collect();
            let mut vein_counts: Vec<f32> = (0..15_i32)
                .map(|i| *theme_proto.vein_count.get((i - 1) as usize).unwrap_or(&0.0))
                .collect();
            let mut vein_opacities: Vec<f32> = (0..15_i32)
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
                    vein_spots[9] += 2;
                    add_until(vein_spots.get_mut(9).unwrap(), 0.45);
                    vein_counts[9] = 0.7;
                    vein_opacities[9] = 1.0;
                    vein_spots[10] += 2;
                    add_until(vein_spots.get_mut(10).unwrap(), 0.45);
                    vein_counts[10] = 0.7;
                    vein_opacities[10] = 1.0;
                    vein_spots[12] += 1;
                    add_until(vein_spots.get_mut(12).unwrap(), 0.5);
                    vein_counts[12] = 0.7;
                    vein_opacities[12] = 0.3;
                    3.5
                }
                StarType::NeutronStar => {
                    vein_spots[14] += 1;
                    add_until(vein_spots.get_mut(14).unwrap(), 0.65);
                    vein_counts[14] = 0.7;
                    vein_opacities[14] = 0.3;
                    4.5
                }
                StarType::BlackHole => {
                    vein_spots[14] += 1;
                    add_until(vein_spots.get_mut(14).unwrap(), 0.65);
                    vein_counts[14] = 0.7;
                    vein_opacities[14] = 0.3;
                    5.0
                }
            };
            let is_rare_resource = self.game_desc.is_rare_resource();
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
                let rare_vein = *rare_vein_ref as usize;
                let rare_vein_chance = theme_proto.rare_settings
                    [index1 * 4 + (if self.star.is_birth() { 0 } else { 1 })];
                let rare_setting_1 = theme_proto.rare_settings[index1 * 4 + 2];
                let rare_setting_2 = theme_proto.rare_settings[index1 * 4 + 3];
                let adjusted_rare_chance =
                    1.0 - (1.0 - rare_vein_chance).powf(star_type_multiplier);
                let adjusted_rare_count = 1.0 - (1.0 - rare_setting_2).powf(star_type_multiplier);
                if rand1.next_f64() < (adjusted_rare_chance as f64) {
                    vein_spots[rare_vein] += 1;
                    vein_counts[rare_vein] = adjusted_rare_count;
                    vein_opacities[rare_vein] = adjusted_rare_count;
                    for _ in 1..12 {
                        if rand1.next_f64() >= (rare_setting_1 as f64) {
                            break;
                        }
                        vein_spots[rare_vein] += 1;
                    }
                }
            }
            let is_infinite_resource = self.game_desc.is_infinite_resource();
            for index3 in 1..15 {
                let vein_spot_count = vein_spots[index3 as usize];
                if vein_spot_count > 0 {
                    let vein_type: VeinType = unsafe { ::std::mem::transmute(index3) };
                    let mut vein = EstimatedVein::new();
                    vein.vein_type = vein_type;
                    vein.min_group = vein_spot_count - 1;
                    vein.max_group = vein_spot_count + 1;
                    if vein.vein_type == VeinType::Oil {
                        vein.min_patch = 1;
                        vein.max_patch = 1;
                    } else {
                        let vein_count_factor = vein_counts[index3 as usize];
                        vein.min_patch = (vein_count_factor * 20.0).round_ties_even() as i32;
                        vein.max_patch = (vein_count_factor * 24.0).round_ties_even() as i32;
                    }
                    let total_amount_factor = if vein.vein_type == VeinType::Oil {
                        f.powf(0.5)
                    } else {
                        f
                    };
                    if is_infinite_resource && vein.vein_type != VeinType::Oil {
                        vein.min_amount = 1;
                        vein.max_amount = 1;
                    } else {
                        let base_amount =
                            ((vein_opacities[index3 as usize] * 100000.0 * total_amount_factor)
                                .round_ties_even() as i32)
                                .max(20);
                        let amount_variance = if base_amount < 16000 {
                            ((base_amount as f32) * (15.0 / 16.0)).floor() as i32
                        } else {
                            15000
                        };
                        let map_amount = |amount: i32| -> i32 {
                            let x1 = ((amount as f32) * 1.1).round_ties_even();
                            let x2 = (if vein.vein_type == VeinType::Oil {
                                x1 * self.game_desc.oil_amount_multiplier()
                            } else {
                                x1 * self.game_desc.resource_multiplier
                            })
                            .round_ties_even() as i32;
                            x2.max(1)
                        };
                        vein.min_amount = map_amount(base_amount - amount_variance);
                        vein.max_amount = map_amount(base_amount + amount_variance);
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
        let orbit_phase_time = time / self.get_orbital_period() + (self.orbit_phase as f64) / 360.0;
        let orbit_cycle = (orbit_phase_time + 0.1) as i32;
        let orbit_fraction = orbit_phase_time - (orbit_cycle as f64);
        let orbit_angle = orbit_fraction * 2.0 * PI_F64;

        let orbit_radius = self.get_orbital_radius() as f64;
        let local_pos = VectorF3(
            (orbit_angle.cos() * orbit_radius) as f32,
            0.0,
            (orbit_angle.sin() * orbit_radius) as f32,
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
        let rotation_phase_time =
            time / self.get_rotation_period() + (self.rotation_phase as f64) / 360.0;
        let rotation_cycle = (rotation_phase_time + 0.1) as i32;
        let rotation_angle = (rotation_phase_time - (rotation_cycle as f64)) * 360.0;

        let rotation = self.get_runtime_system_rotation()
            * Quaternion::angle_axis(rotation_angle as f32, &VectorF3::down());

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
        algo_id: i32,
        vein_type: &VeinType,
        zero: &VectorF3,
        raw_data: &mut PlanetRawData,
    ) -> bool {
        if algo_id == 7 && vein_type != &VeinType::Bamboo {
            return true;
        }
        // `zero` is already normalized by the caller
        let height = raw_data.query_height_normalized(zero);
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
                return Vec::with_capacity(0);
            }

            let theme = self.get_theme();
            let mut rand1 = DspRandom::new(self.seed);
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            rand1.next_f64();
            let birth_seed = rand1.next_seed();
            let mut rand2 = DspRandom::new(rand1.next_seed());
            let mut vein_spots: Vec<i32> = (0..15_i32)
                .map(|i| *theme.vein_spot.get((i - 1) as usize).unwrap_or(&0))
                .collect();
            let mut vein_counts: Vec<f32> = (0..15_i32)
                .map(|i| *theme.vein_count.get((i - 1) as usize).unwrap_or(&0.0))
                .collect();
            let mut vein_opacities: Vec<f32> = (0..15_i32)
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
                    vein_spots[9] += 2;
                    add_until(vein_spots.get_mut(9).unwrap(), 0.45);
                    vein_counts[9] = 0.7;
                    vein_opacities[9] = 1.0;
                    vein_spots[10] += 2;
                    add_until(vein_spots.get_mut(10).unwrap(), 0.45);
                    vein_counts[10] = 0.7;
                    vein_opacities[10] = 1.0;
                    vein_spots[12] += 1;
                    add_until(vein_spots.get_mut(12).unwrap(), 0.5);
                    vein_counts[12] = 0.7;
                    vein_opacities[12] = 0.3;
                    3.5
                }
                StarType::NeutronStar => {
                    vein_spots[14] += 1;
                    add_until(vein_spots.get_mut(14).unwrap(), 0.65);
                    vein_counts[14] = 0.7;
                    vein_opacities[14] = 0.3;
                    4.5
                }
                StarType::BlackHole => {
                    vein_spots[14] += 1;
                    add_until(vein_spots.get_mut(14).unwrap(), 0.65);
                    vein_counts[14] = 0.7;
                    vein_opacities[14] = 0.3;
                    5.0
                }
            };

            for (index1, rare_vein_ref) in theme.rare_veins.iter().enumerate() {
                let rare_vein = *rare_vein_ref as usize;
                let rare_vein_chance =
                    theme.rare_settings[index1 * 4 + (if self.star.is_birth() { 0 } else { 1 })];
                let rare_setting_1 = theme.rare_settings[index1 * 4 + 2];
                let rare_setting_2 = theme.rare_settings[index1 * 4 + 3];
                let adjusted_rare_chance =
                    1.0 - (1.0 - rare_vein_chance).powf(star_type_multiplier);
                let adjust_rare_count = 1.0 - (1.0 - rare_setting_2).powf(star_type_multiplier);
                if rand1.next_f64() < (adjusted_rare_chance as f64) {
                    vein_spots[rare_vein] += 1;
                    vein_counts[rare_vein] = adjust_rare_count;
                    vein_opacities[rare_vein] = adjust_rare_count;
                    for _ in 1..12 {
                        if rand1.next_f64() >= (rare_setting_1 as f64) {
                            break;
                        }
                        vein_spots[rare_vein] += 1;
                    }
                }
            }

            let is_rare_resource = self.game_desc.is_rare_resource();
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
            // Fetch PlanetRawData once and thread it through all query_height calls
            let mut raw_data = PlanetRawData::new(&self);

            let birth_point = if is_birth_planet {
                let star_direction = self.get_star_direction();
                let birth_point_data =
                    BirthPoints::new(&mut raw_data, birth_seed, self.radius, star_direction);
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

            let is_infinite_resource = self.game_desc.is_infinite_resource();
            // Fixed array indexed by VeinType discriminant (0..16) — avoids HashMap hashing overhead
            let mut amount_map: [i32; 16] = [0; 16];

            let min_vein_spacing = 2.1 / self.radius;
            let min_vein_spacing_sq = (min_vein_spacing as f64) * (min_vein_spacing as f64);
            let algo_id = self.get_algo_id();

            for index3 in 1..15 {
                if vein_vectors.len() >= 512 {
                    break;
                }
                let mut vein_spot_count = vein_spots[index3 as usize];
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
                        let mut normal_dir = VectorF3(x as f32, y as f32, z as f32);
                        if vein_type != VeinType::Oil {
                            normal_dir += birth_point;
                        }
                        normal_dir.normalize();
                        if self.can_place_vein(algo_id, &vein_type, &normal_dir, &mut raw_data) {
                            let not_too_close_to_other_vein =
                                vein_vectors.iter().all(|(_, pos, _)| {
                                    (pos.distance_sq_from(&normal_dir) as f64) >= min_sq_dist
                                });
                            if not_too_close_to_other_vein {
                                vein_vectors.push((vein_type, normal_dir, false));
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
                let right_axis = &rotation * &VectorF3::right();
                let forward_axis = &rotation * &VectorF3::forward();
                let vein_type_index = *vein_type as i32;
                let target_node_count = if *is_birth_resource {
                    rand2.next_f64();
                    6
                } else if is_oil {
                    rand2.next_f64();
                    1
                } else {
                    (vein_counts.get(vein_type_index as usize).unwrap()
                        * (rand2.next_i32(5) + 20) as f32)
                        .round_ties_even() as usize
                };
                let mut vein_nodes = Vec::with_capacity(target_node_count);
                vein_nodes.push(VectorF2::zero());
                let vein_density = if *is_birth_resource {
                    0.2_f32
                } else {
                    *vein_opacities.get(vein_type_index as usize).unwrap()
                };
                for _ in 0..20 {
                    if vein_nodes.len() >= target_node_count {
                        break;
                    }
                    for index8 in 0..vein_nodes.len() {
                        let existing_node = vein_nodes.get(index8).unwrap();
                        if existing_node.magnitude_sq() <= 36.0 {
                            let random_angle_radians = rand2.next_f64() * PI * 2.0;
                            let mut random_dir = VectorF2::new(
                                random_angle_radians.cos() as f32,
                                random_angle_radians.sin() as f32,
                            );
                            random_dir += existing_node * 0.2;
                            random_dir.normalize();
                            let new_node = existing_node + &random_dir;
                            let not_too_close_to_other_node = vein_nodes
                                .iter()
                                .all(|v| v.distance_sq_from(&new_node) >= 0.85);
                            if not_too_close_to_other_node {
                                vein_nodes.push(new_node);
                            }
                            if vein_nodes.len() >= target_node_count {
                                break;
                            }
                        }
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
                for pos in vein_nodes.iter() {
                    let raw_amount = rand2.next_i32(value_range) + min_value;
                    let amount = if is_infinite_resource && !is_oil {
                        1
                    } else {
                        let multiplier = if is_oil {
                            self.game_desc.oil_amount_multiplier()
                        } else {
                            self.game_desc.resource_multiplier
                        };
                        (((raw_amount as f32) * 1.1 * multiplier).round_ties_even() as i32).max(1)
                    };
                    if algo_id == 7 || theme.water_item_id == 0 {
                        amount_map[*vein_type as usize] += amount;
                    } else {
                        let node_offset =
                            ((right_axis * pos.0) + (forward_axis * pos.1)) * min_vein_spacing;
                        let mut pos = normalized + node_offset;
                        if is_oil {
                            pos = self.snap_to(&pos);
                        }
                        let surface_height = raw_data.query_height(&pos);
                        if surface_height >= self.radius {
                            // println!("{:?},{:?},{}", pos * surface_height, vein_type, amount);
                            amount_map[*vein_type as usize] += amount;
                        }
                    }
                }
            }

            amount_map
                .iter()
                .enumerate()
                .filter(|(_, &amount)| amount > 0)
                .map(|(i, &amount)| ActualVein {
                    vein_type: unsafe { ::std::mem::transmute(i as i32) },
                    amount,
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
        let latitude_angle = pos.1.asin();
        let mut longitude_angle = pos.0.atan2(-pos.2);
        let mut latitude_index_raw = latitude_angle / two_pi * segment;
        let latitude_index = (latitude_index_raw.abs() - 0.1).max(0.0) as i32;
        let longitude_segment_count =
            determine_longitude_segment_count(latitude_index, segment) as f32;
        let mut longitude_index_raw = longitude_angle / two_pi * longitude_segment_count;
        latitude_index_raw = (latitude_index_raw * 5.0).round_ties_even() / 5.0;
        longitude_index_raw = (longitude_index_raw * 5.0).round_ties_even() / 5.0;
        let latitude_radians = latitude_index_raw / segment * two_pi;
        longitude_angle = longitude_index_raw / longitude_segment_count * two_pi;
        let latitude_sin = latitude_radians.sin();
        let latitude_cos = latitude_radians.cos();
        let longitude_sin = longitude_angle.sin();
        let longitude_cos = longitude_angle.cos();
        VectorF3(
            latitude_cos * longitude_sin,
            latitude_sin,
            latitude_cos * (-longitude_cos),
        )
    }
}

fn determine_longitude_segment_count(latitude_index: i32, segment: f32) -> i32 {
    let candidate_segment_count = (((latitude_index as f32) / (segment / 4.0) * PI as f32 * 0.5)
        .cos()
        .abs()
        * segment)
        .ceil() as usize;
    if candidate_segment_count < 500 {
        SEGMENT_TABLE[candidate_segment_count]
    } else {
        ((candidate_segment_count as i32) + 49) / 100 * 100
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
        if self.game_desc.use_actual_veins {
            state.serialize_field("actualVeins", &self.get_actual_veins())?;
        } else {
            state.serialize_field("veins", &self.get_estimated_veins())?;
        }
        state.end()
    }
}
