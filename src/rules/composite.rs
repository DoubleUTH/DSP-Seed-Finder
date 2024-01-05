use crate::data::rule::{Condition, Rule};

pub struct RuleComposite {
    pub rule: Box<dyn Rule + Send>,
    pub condition: Condition,
}

impl Rule for RuleComposite {
    fn get_priority(&self) -> i32 {
        self.rule.get_priority()
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let result = self.rule.evaluate(galaxy, evaluation);
        if self.condition.eval(result.len() as f32) {
            return vec![0];
        }
        vec![]
    }
}

pub struct RuleCompositeAnd {
    pub rules: Vec<Box<dyn Rule + Send>>,
}

impl Rule for RuleCompositeAnd {
    fn get_priority(&self) -> i32 {
        self.rules
            .iter()
            .map(|rule| rule.get_priority())
            .max()
            .unwrap_or_default()
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        for rule in &self.rules {
            let result = rule.evaluate(galaxy, evaluation);
            if result.is_empty() {
                return result;
            }
        }
        vec![0]
    }
}

pub struct RuleCompositeOr {
    pub rules: Vec<Box<dyn Rule + Send>>,
}

impl Rule for RuleCompositeOr {
    fn get_priority(&self) -> i32 {
        self.rules
            .iter()
            .map(|rule| rule.get_priority())
            .max()
            .unwrap_or_default()
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        for rule in &self.rules {
            let result = rule.evaluate(galaxy, evaluation);
            if !result.is_empty() {
                return result;
            }
        }
        vec![]
    }
}
