use super::planet::Planet;
use super::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Condition {
    Eq(f32),
    Neq(f32),
    Lt(f32),
    Lte(f32),
    Gt(f32),
    Gte(f32),
    Between(f32, f32),
    NotBetween(f32, f32),
}

impl Condition {
    pub fn eval(&self, value: f32) -> bool {
        match self {
            Condition::Eq(f) => value == *f,
            Condition::Neq(f) => value != *f,
            Condition::Lt(f) => value < *f,
            Condition::Lte(f) => value <= *f,
            Condition::Gt(f) => value > *f,
            Condition::Gte(f) => value >= *f,
            Condition::Between(f1, f2) => *f1 <= value && value <= *f2,
            Condition::NotBetween(f1, f2) => *f1 > value || value > *f2,
        }
    }
}

#[allow(unused_variables)]
pub trait Rule {
    fn is_evaluated(&self) -> bool;
    // optimization for birth only
    fn is_birth(&self) -> bool {
        false
    }
    fn on_planets_created(&mut self, star: &Star, planets: &Vec<Planet>) -> Option<bool> {
        None
    }
    fn on_planets_themed(&mut self, star: &Star, planets: &Vec<Planet>) -> Option<bool> {
        None
    }
    fn on_veins_generated(&mut self, star: &Star, planets: &Vec<Planet>) -> Option<bool> {
        None
    }
    fn reset(&mut self) -> ();
}
