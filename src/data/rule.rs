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

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluaton) -> Vec<usize> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct Evaluaton {
    items: Vec<Option<bool>>,
    max_len: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleTarget {
    Galaxy,
    Star,
    Planet,
}

impl Evaluaton {
    pub fn new(size: usize) -> Self {
        Self {
            items: (0..size).map(|_| None).collect(),
            max_len: size,
        }
    }

    pub fn get_result(&self, index: usize) -> Option<bool> {
        self.items[index]
    }

    pub fn is_unknonwn(&self, index: usize) -> bool {
        self.get_result(index).is_none()
    }

    pub fn is_known(&self, index: usize) -> bool {
        self.get_result(index).is_some()
    }

    /// Returns the minium number of stars that still needs to be evaluated
    /// Useful for unsafe rules
    pub fn get_len(&self) -> usize {
        self.max_len
    }

    fn load_max_len(&mut self) {
        let mut x = self.max_len - 1;
        loop {
            if self.items[x].is_none() {
                self.max_len = x + 1;
                return;
            }
            if x == 0 {
                self.max_len = 0;
                return;
            }
            x -= 1;
        }
    }

    pub fn confirm_many(&mut self, indices: &Vec<usize>) {
        for index in indices {
            let item = self.items.get_mut(*index).unwrap();
            if item.is_none() {
                *item = Some(true);
            }
        }
        self.load_max_len();
    }

    pub fn reject_others(&mut self, indices: &Vec<usize>) {
        for (index, val) in self.items.iter_mut().enumerate() {
            if val.is_none() && !indices.contains(&index) {
                *val = Some(false)
            }
        }
        self.load_max_len();
    }

    pub fn collect_known(&self) -> Vec<usize> {
        self.items
            .iter()
            .enumerate()
            .filter(|(_, &item)| item == Some(true))
            .map(|(index, _)| index)
            .collect()
    }

    pub fn collect_unknown(&self) -> Vec<usize> {
        self.items
            .iter()
            .enumerate()
            .filter(|(_, &item)| item != Some(false))
            .map(|(index, _)| index)
            .collect()
    }

    pub fn is_done(&self) -> bool {
        self.max_len == 0
    }
}
