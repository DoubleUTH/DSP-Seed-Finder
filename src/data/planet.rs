use super::enums::{PlanetType, SpectrType, StarType, ThemeDistribute, VeinType};
use super::random::DspRandom;
use super::star::Star;
use super::theme_proto::{ThemeProto, THEME_PROTOS};
use super::vein::Vein;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::cell::{OnceCell, RefCell};
use std::f64::consts::PI;
use std::rc::Rc;

#[derive(Debug)]
pub struct Planet<'a> {
    pub star: Rc<Star<'a>>,
    pub index: usize,
    pub seed: i32,
    #[expect(unused)]
    pub info_seed: i32,
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
    orbital_radius: OnceCell<f32>,
    sun_distance: OnceCell<f32>,
    temperature_factor: OnceCell<f32>,
    habitable_bias: OnceCell<f32>,
    temperature_bias: OnceCell<f32>,
    luminosity: OnceCell<f32>,
    unmodified_planet_type: OnceCell<PlanetType>,
    orbit_inclination: OnceCell<f32>,
    sun_orbital_period: OnceCell<f64>,
    orbital_period: OnceCell<f64>,
    obliquity: OnceCell<f32>,
    eligible_for_resonance: OnceCell<bool>,
    rotation_period: OnceCell<f64>,
    theme: OnceCell<&'static ThemeProto>,
    gases: OnceCell<Vec<(i32, f32)>>,
    veins: OnceCell<Vec<Vein>>,
}

const ORBIT_RADIUS: &[f32] = &[
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
        rand.next_f64();
        rand.next_f64();
        rand.next_f64();
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
            info_seed,
            theme_seed,
            orbit_around: RefCell::new(None),
            orbit_index,
            radius,
            scale,
            orbit_longitude,
            orbit_phase,
            rotation_phase,
            theme_rand1,
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
            habitable_bias: OnceCell::new(),
            temperature_bias: OnceCell::new(),
            luminosity: OnceCell::new(),
            unmodified_planet_type: OnceCell::new(),
            orbit_inclination: OnceCell::new(),
            sun_orbital_period: OnceCell::new(),
            orbital_period: OnceCell::new(),
            obliquity: OnceCell::new(),
            eligible_for_resonance: OnceCell::new(),
            rotation_period: OnceCell::new(),
            theme: OnceCell::new(),
            gases: OnceCell::new(),
            veins: OnceCell::new(),
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

    pub fn get_habitable_bias(&self) -> f32 {
        *self.habitable_bias.get_or_init(|| {
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
        })
    }

    pub fn get_temperature_bias(&self) -> f32 {
        *self.temperature_bias.get_or_init(|| {
            if self.is_gas_giant() {
                0.0
            } else {
                let f2 = self.get_temperature_factor();
                (1.2 / ((f2 as f64) + 0.2) - 1.0) as f32
            }
        })
    }

    pub fn get_luminosity(&self) -> f32 {
        *self.luminosity.get_or_init(|| {
            let mut luminosity =
                (self.star.get_light_balance_radius() / (self.get_sun_distance() + 0.01)).powf(0.6);
            if luminosity > 1.0 {
                luminosity = luminosity.ln() + 1.0;
                luminosity = luminosity.ln() + 1.0;
                luminosity = luminosity.ln() + 1.0;
            }
            (luminosity * 100.0).round() / 100.0
        })
    }

    fn increment_habitable_count(&self) {
        self.star
            .game_desc
            .habitable_count
            .set(self.star.game_desc.habitable_count.get() + 1);
    }

    pub fn get_unmodified_planet_type(&self) -> &PlanetType {
        self.unmodified_planet_type.get_or_init(|| {
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
                        PlanetType::Vocano
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
        })
    }

    pub fn is_tidal_locked(&self) -> bool {
        self.get_rotation_period() == self.get_orbital_period()
    }

    pub fn get_orbit_inclination(&self) -> f32 {
        *self.orbit_inclination.get_or_init(|| {
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
        })
    }

    pub fn get_sun_orbital_period(&self) -> f64 {
        *self.sun_orbital_period.get_or_init(|| {
            if let Some(orbit_planet) = self.orbit_around.borrow().as_deref() {
                orbit_planet.get_orbital_period()
            } else {
                self.get_orbital_period()
            }
        })
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
                if self.star.is_birth() && planet_type == &PlanetType::Ocean {
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
                    if (theme.planet_type == *planet_type) && flag2 {
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

    pub fn get_veins(&self) -> &Vec<Vein> {
        self.veins.get_or_init(|| {
            let mut output: Vec<Vein> = vec![];
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
                    let mut vein = Vein::new();
                    vein.vein_type = vein_type;
                    vein.min_group = num8 - 1;
                    vein.max_group = num8 + 1;
                    if vein.vein_type == VeinType::Oil {
                        vein.min_patch = 1;
                        vein.max_patch = 1;
                    } else {
                        let num12 = num_array_2[index3 as usize];
                        vein.min_patch = (num12 * 20.0).round() as i32;
                        vein.max_patch = (num12 * 24.0).round() as i32;
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
                        let num17 = ((num_array_3[index3 as usize] * 100000.0 * num16).round()
                            as i32)
                            .max(20);
                        let num18 = if num17 < 16000 {
                            ((num17 as f32) * (15.0 / 16.0)).floor() as i32
                        } else {
                            15000
                        };
                        let map_amount = |amount: i32| -> i32 {
                            let x1 = ((amount as f32) * 1.1).round();
                            let x2 = (if vein.vein_type == VeinType::Oil {
                                x1 * self.star.game_desc.oil_amount_multipler()
                            } else {
                                x1 * self.star.game_desc.resource_multiplier
                            })
                            .round() as i32;
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
}

impl Serialize for Planet<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Planet", 16)?;
        state.serialize_field("index", &self.index)?;
        state.serialize_field("orbitAround", &self.orbit_around.borrow().map(|p| p.index))?;
        state.serialize_field("orbitIndex", &self.orbit_index)?;
        state.serialize_field("orbitRadius", &self.get_orbital_radius())?;
        state.serialize_field("orbitInclination", &self.get_orbit_inclination())?;
        state.serialize_field("orbitLongitude", &self.orbit_longitude)?;
        state.serialize_field("orbitalPeriod", &self.get_orbital_period())?;
        state.serialize_field("orbitPhase", &self.orbit_phase)?;
        state.serialize_field("obliquity", &self.get_obliquity())?;
        state.serialize_field("rotationPeriod", &self.get_rotation_period())?;
        state.serialize_field("rotationPhase", &self.rotation_phase)?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("luminosity", &self.get_luminosity())?;
        state.serialize_field("theme", &self.get_theme())?;
        state.serialize_field("veins", &self.get_veins())?;
        state.serialize_field("gases", &self.get_gases())?;
        state.end()
    }
}
