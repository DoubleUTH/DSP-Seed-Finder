use crate::data::planet::Planet;
use crate::data::rule::Rule;
use crate::data::star::Star;

pub struct RuleAnd {
    pub evaluated: bool,
    pub rules: Vec<Box<dyn Rule>>,
}

impl Rule for RuleAnd {
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn on_star_created(&mut self, star: &Star) -> Option<bool> {
        let mut has_unknown = false;
        for rule in self.rules.iter_mut() {
            if !rule.is_evaluated() {
                match rule.on_star_created(star) {
                    Some(false) => {
                        self.evaluated = true;
                        return Some(false);
                    }
                    None => {
                        has_unknown = true;
                    }
                    _ => {}
                };
            }
        }
        if has_unknown { None } else { self.evaluated = true; Some(true) }
    }
    fn on_planets_created(&mut self, star: &Star, planets: &Vec<Planet>) -> Option<bool> {
        let mut has_unknown = false;
        for rule in self.rules.iter_mut() {
            if !rule.is_evaluated() {
                match rule.on_planets_created(star, planets) {
                    Some(false) => {
                        self.evaluated = true;
                        return Some(false);
                    }
                    None => {
                        has_unknown = true;
                    }
                    _ => {}
                };
            }
        }
        if has_unknown { None } else { self.evaluated = true; Some(true) }
    }
    fn on_planets_themed(&mut self, star: &Star, planets: &Vec<Planet>) -> Option<bool> {
        let mut has_unknown = false;
        for rule in self.rules.iter_mut() {
            if !rule.is_evaluated() {
                match rule.on_planets_themed(star, planets) {
                    Some(false) => {
                        self.evaluated = true;
                        return Some(false);
                    }
                    None => {
                        has_unknown = true;
                    }
                    _ => {}
                };
            }
        }
        if has_unknown { None } else { self.evaluated = true; Some(true) }
    }
    fn on_veins_generated(&mut self, star: &Star, planets: &Vec<Planet>) -> Option<bool> {
        let mut has_unknown = false;
        for rule in self.rules.iter_mut() {
            if !rule.is_evaluated() {
                match rule.on_veins_generated(star, planets) {
                    Some(false) => {
                        self.evaluated = true;
                        return Some(false);
                    }
                    None => {
                        has_unknown = true;
                    }
                    _ => {}
                };
            }
        }
        if has_unknown { None } else { self.evaluated = true; Some(true) }
    }
    fn reset(&mut self) {
        self.evaluated = false;
        for rule in self.rules.iter_mut() {
            rule.reset();
        }
    }
}
