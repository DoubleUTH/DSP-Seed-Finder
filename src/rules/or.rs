use crate::data::planet::Planet;
use crate::data::rule::Rule;
use crate::data::star::Star;

pub struct RuleOr {
    pub evaluated: bool,
    pub rules: Vec<Box<dyn Rule>>,
}

impl Rule for RuleOr {
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn on_star_created(&mut self, star: &Star) -> Option<bool> {
        let mut has_unknown = false;
        for rule in self.rules.iter_mut() {
            if !rule.is_evaluated() {
                match rule.on_star_created(star) {
                    Some(true) => {
                        self.evaluated = true;
                        return Some(true);
                    }
                    None => {
                        has_unknown = true;
                    }
                    _ => {}
                };
            }
        }
        if has_unknown { None } else { Some(false) }
    }
    fn on_planets_created(&mut self, star: &Star, planets: &Vec<Planet>) -> Option<bool> {
        let mut has_unknown = false;
        for rule in self.rules.iter_mut() {
            if !rule.is_evaluated() {
                match rule.on_planets_created(star, planets) {
                    Some(true) => {
                        self.evaluated = true;
                        return Some(true);
                    }
                    None => {
                        has_unknown = true;
                    }
                    _ => {}
                };
            }
        }
        if has_unknown { None } else { Some(false) }
    }
    fn on_planets_themed(&mut self, star: &Star, planets: &Vec<Planet>) -> Option<bool> {
        let mut has_unknown = false;
        for rule in self.rules.iter_mut() {
            if !rule.is_evaluated() {
                match rule.on_planets_themed(star, planets) {
                    Some(true) => {
                        self.evaluated = true;
                        return Some(true);
                    }
                    None => {
                        has_unknown = true;
                    }
                    _ => {}
                };
            }
        }
        if has_unknown { None } else { Some(false) }
    }
    fn on_veins_generated(&mut self, star: &Star, planets: &Vec<Planet>) -> Option<bool> {
        let mut has_unknown = false;
        for rule in self.rules.iter_mut() {
            if !rule.is_evaluated() {
                match rule.on_veins_generated(star, planets) {
                    Some(true) => {
                        self.evaluated = true;
                        return Some(true);
                    }
                    None => {
                        has_unknown = true;
                    }
                    _ => {}
                };
            }
        }
        if has_unknown { None } else { Some(false) }
    }
    fn reset(&mut self) {
        self.evaluated = false;
        for rule in self.rules.iter_mut() {
            rule.reset();
        }
    }
}
