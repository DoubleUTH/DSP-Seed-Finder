use super::enums::PlanetType;
use super::theme_proto::{ThemeProto, DEFAULT_THEME_PROTO};
use super::vein::Vein;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Planet {
    pub index: i32,
    pub seed: i32,
    pub info_seed: i32,
    pub theme_seed: i32,
    pub is_birth: bool,
    pub orbit_around: Option<i32>,
    pub orbit_index: i32,
    pub radius: f32,
    pub scale: f32,
    pub orbit_radius: f32,
    pub orbit_inclination: f32,
    pub orbit_longitude: f32,
    pub orbital_period: f64,
    pub sun_orbital_period: f64,
    pub orbit_phase: f32,
    obliquity: Cell<f32>,
    rotation_period: Cell<f64>,
    pub rotation_phase: f32,
    rotation_calculated: Cell<bool>,
    pub rotation_params: (f64, f64, f64),
    pub sun_distance: f32,
    pub planet_type: PlanetType,
    pub habitable_bias: f32,
    pub temperature_bias: f32,
    pub star_light_balance_radius: f32,
    luminosity: Cell<f32>,
    luminosity_calculated: Cell<bool>,
    pub theme_proto: &'static ThemeProto,
    pub theme_rand1: f64,
    pub veins: Vec<Vein>,
    pub gases: Vec<(i32, f32)>,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            index: 0,
            seed: 0,
            info_seed: 0,
            theme_seed: 0,
            is_birth: false,
            orbit_around: None,
            orbit_index: 0,
            radius: 200.0,
            scale: 1.0,
            orbit_radius: 0.0,
            orbit_inclination: 0.0,
            orbit_longitude: 0.0,
            orbital_period: 0.0,
            sun_orbital_period: 0.0,
            orbit_phase: 0.0,
            obliquity: Cell::new(0.0),
            rotation_period: Cell::new(0.0),
            rotation_phase: 0.0,
            rotation_calculated: Cell::new(false),
            rotation_params: (0.0, 0.0, 0.0),
            sun_distance: 0.0,
            planet_type: PlanetType::None,
            habitable_bias: 0.0,
            temperature_bias: 0.0,
            star_light_balance_radius: 0.0,
            luminosity: Cell::new(0.0),
            luminosity_calculated: Cell::new(false),
            theme_proto: DEFAULT_THEME_PROTO,
            theme_rand1: 0.0,
            veins: vec![],
            gases: vec![],
        }
    }
}

impl Planet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn real_radius(&self) -> f32 {
        self.radius * self.scale
    }

    fn calculate_luminosity(&self) {
        if self.luminosity_calculated.get() {
            return;
        }
        self.luminosity_calculated.set(true);
        let mut luminosity =
            (self.star_light_balance_radius / (self.sun_distance + 0.01)).powf(0.6);
        if luminosity > 1.0 {
            luminosity = luminosity.ln() + 1.0;
            luminosity = luminosity.ln() + 1.0;
            luminosity = luminosity.ln() + 1.0;
        }
        luminosity = (luminosity * 100.0).round() / 100.0;
        self.luminosity.set(luminosity);
    }

    pub fn get_luminosity(&self) -> f32 {
        self.calculate_luminosity();
        self.luminosity.get()
    }

    pub fn is_tidal_locked(&self) -> bool {
        self.get_rotation_period() == self.orbital_period
    }

    fn calculate_rotation(&self) {
        if self.rotation_calculated.get() {
            return;
        }
        self.rotation_calculated.set(true);
        let (obliquity_scale, num15, rotation_scale) = self.rotation_params;
        let mut obliquity: f32;
        if num15 < 0.04 {
            obliquity = (obliquity_scale * 39.9) as f32;
            if obliquity < 0.0 {
                obliquity -= 70.0;
            } else {
                obliquity += 70.0;
            }
        } else if num15 < 0.1 {
            obliquity = (obliquity_scale * 80.0) as f32;
            if obliquity < 0.0 {
                obliquity -= 30.0;
            } else {
                obliquity += 30.0;
            }
        } else {
            obliquity = (obliquity_scale * 60.0) as f32;
        }
        let gas_giant = self.planet_type == PlanetType::Gas;
        let mut rotation_period = rotation_scale
            * (if self.orbit_around.is_none() {
                self.orbit_radius.powf(0.25) as f64
            } else {
                1.0
            })
            * (if gas_giant { 0.2 } else { 1.0 });

        rotation_period = 1.0 / (1.0 / self.sun_orbital_period + 1.0 / rotation_period);
        if self.orbit_around.is_none() && self.orbit_index <= 4 && !gas_giant {
            if num15 > 0.96 {
                obliquity *= 0.01;
                rotation_period = self.orbital_period;
            } else if num15 > 0.930000007152557 {
                obliquity *= 0.1;
                rotation_period = self.orbital_period * 0.5;
            } else if num15 > 0.9 {
                obliquity *= 0.2;
                rotation_period = self.orbital_period * 0.25;
            }
        }

        if num15 > 0.85 && num15 <= 0.9 {
            rotation_period = -rotation_period;
        }
        self.obliquity.set(obliquity);
        self.rotation_period.set(rotation_period);
    }

    pub fn get_rotation_period(&self) -> f64 {
        self.calculate_rotation();
        self.rotation_period.get()
    }

    pub fn get_obliquity(&self) -> f32 {
        self.calculate_rotation();
        self.obliquity.get()
    }
}

impl Serialize for Planet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Planet", 17)?;
        state.serialize_field("index", &self.index)?;
        state.serialize_field("orbitAround", &self.orbit_around)?;
        state.serialize_field("orbitIndex", &self.orbit_index)?;
        state.serialize_field("orbitRadius", &self.orbit_radius)?;
        state.serialize_field("orbitInclination", &self.orbit_inclination)?;
        state.serialize_field("orbitLongitude", &self.orbit_longitude)?;
        state.serialize_field("orbitalPeriod", &self.orbital_period)?;
        state.serialize_field("orbitPhase", &self.orbit_phase)?;
        state.serialize_field("obliquity", &self.get_obliquity())?;
        state.serialize_field("rotationPeriod", &self.get_rotation_period())?;
        state.serialize_field("rotationPhase", &self.rotation_phase)?;
        state.serialize_field("sunDistance", &self.sun_distance)?;
        state.serialize_field("type", &self.planet_type)?;
        state.serialize_field("luminosity", &self.get_luminosity())?;
        state.serialize_field("theme", &self.theme_proto)?;
        state.serialize_field("veins", &self.veins)?;
        state.serialize_field("gases", &self.gases)?;
        state.end()
    }
}
