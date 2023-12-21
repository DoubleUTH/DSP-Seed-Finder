use super::planet::Planet;
use super::star::Star;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    In(Vec<usize>),
    NotIn(Vec<usize>),
    Exist,
    NotExist,
}

impl Condition {
    pub fn eval(&self, value: f32) -> Option<bool> {
        match self {
            Condition::Eq(f) => Some(value == *f),
            Condition::Neq(f) => Some(value != *f),
            Condition::Lt(f) => Some(value < *f),
            Condition::Lte(f) => Some(value <= *f),
            Condition::Gt(f) => Some(value > *f),
            Condition::Gte(f) => Some(value >= *f),
            Condition::Between(f1, f2) => Some(*f1 <= value && value <= *f2),
            Condition::NotBetween(f1, f2) => Some(*f1 > value || value > *f2),
            _ => None,
        }
    }
}

#[allow(unused_variables)]
#[typetag::serde(tag = "type")]
pub trait Rule {
    fn is_evaluated(&self) -> bool;
    fn on_star_created(&mut self, star: &Star) -> Option<bool> {
        None
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
