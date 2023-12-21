use super::enums::{SpectrType, StarType};
use super::planet::Planet;
use super::vector3::Vector3;

#[derive(Debug, Clone)]
pub struct Star {
    pub id: i32,
    pub index: i32,
    pub seed: i32,
    pub position: Vector3,
    pub name: String,
    pub level: f32,
    pub resource_coef: f32,
    pub mass: f32,
    pub lifetime: f32,
    pub age: f32,
    pub temperature: f32,
    pub star_type: StarType,
    pub spectr: SpectrType,
    pub color: f32,
    pub class_factor: f32,
    pub luminosity: f32,
    pub radius: f32,
    pub habitable_radius: f32,
    pub lignt_balance_radius: f32,
    pub orbit_scaler: f32,
    pub dyson_radius: f32,
    pub planet_count: i32,
    pub planets: Vec<Planet>,
}

impl Default for Star {
    fn default() -> Self {
        Self {
            id: 0,
            index: 0,
            seed: 0,
            position: Vector3::zero(),
            name: "".to_owned(),
            level: 0.0,
            resource_coef: 0.0,
            mass: 0.0,
            lifetime: 0.0,
            age: 0.0,
            temperature: 0.0,
            star_type: StarType::MainSeqStar,
            spectr: SpectrType::X,
            color: 0.0,
            class_factor: 0.0,
            luminosity: 0.0,
            radius: 0.0,
            habitable_radius: 0.0,
            lignt_balance_radius: 0.0,
            orbit_scaler: 0.0,
            dyson_radius: 0.0,
            planet_count: 0,
            planets: vec![],
        }
    }
}

impl Star {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn physics_radius(&self) -> f32 {
        return self.radius * 1200.0;
    }

    pub fn astro_id(&self) -> i32 {
        return self.id * 100;
    }
}
