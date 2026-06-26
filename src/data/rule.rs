use super::galaxy::Galaxy;
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
    fn get_priority(&self) -> i32 {
        0
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluaton) -> u64 {
        0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Evaluaton {
    value: u64,
    unknown: u64,
    max_len: usize,
}

impl Evaluaton {
    pub fn new(size: usize) -> Self {
        Self {
            value: 0,
            unknown: u64::MAX >> (u64::BITS - size as u32),
            max_len: size,
        }
    }

    #[inline]
    pub fn is_known(&self, index: usize) -> bool {
        (self.unknown & (1 << index)) == 0
    }

    /// Returns the minium number of stars that still needs to be evaluated
    /// Useful for unsafe rules
    #[inline]
    pub fn get_len(&self) -> usize {
        self.max_len
    }

    #[inline]
    fn load_max_len(&mut self) {
        self.max_len = (u64::BITS - self.unknown.leading_zeros()) as usize;
    }

    #[inline]
    pub fn accept_many(&mut self, indices: u64) {
        self.value |= self.unknown & indices;
        self.unknown &= !indices;
        self.load_max_len();
    }

    #[inline]
    pub fn reject_others(&mut self, indices: u64) {
        self.value &= !self.unknown | indices;
        self.unknown &= indices;
        self.load_max_len();
    }

    #[inline]
    pub fn collect_known(&self) -> u64 {
        !self.unknown & self.value
    }

    #[inline]
    pub fn collect_unknown(&self) -> u64 {
        self.unknown | self.value
    }

    #[inline]
    pub fn is_done(&self) -> bool {
        self.max_len == 0
    }
}
