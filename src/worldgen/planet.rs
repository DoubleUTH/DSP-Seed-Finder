use super::enums::PlanetType;
use super::theme_proto::{ThemeProto, DEFAULT_THEME_PROTO};
use super::vein::Vein;

#[derive(Debug, Clone)]
pub struct Planet {
    pub index: i32,
    pub seed: i32,
    pub info_seed: i32,
    pub theme_seed: i32,
    pub orbit_around: i32,
    pub orbit_index: i32,
    pub number: i32,
    pub id: i32,
    pub name: String,
    pub radius: f32,
    pub scale: f32,
    pub is_birth: bool,
    pub orbit_radius: f32,
    pub orbit_inclination: f32,
    pub orbit_longitude: f32,
    pub orbital_period: f64,
    pub orbit_phase: f32,
    pub obliquity: f32,
    pub rotation_period: f64,
    pub rotation_phase: f32,
    pub sun_distance: f32,
    pub planet_type: PlanetType,
    pub habitable_bias: f32,
    pub temperature_bias: f32,
    pub theme_proto: &'static ThemeProto,
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
            orbit_around: 0,
            orbit_index: 0,
            number: 0,
            id: 0,
            name: "".to_owned(),
            radius: 200.0,
            scale: 1.0,
            is_birth: false,
            orbit_radius: 0.0,
            orbit_inclination: 0.0,
            orbit_longitude: 0.0,
            orbital_period: 0.0,
            orbit_phase: 0.0,
            obliquity: 0.0,
            rotation_period: 0.0,
            rotation_phase: 0.0,
            sun_distance: 0.0,
            planet_type: PlanetType::None,
            habitable_bias: 0.0,
            temperature_bias: 0.0,
            theme_proto: DEFAULT_THEME_PROTO,
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
}
