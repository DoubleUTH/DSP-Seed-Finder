use crate::data::rule::{Condition, Rule};

pub struct MultiRule {
    pub rule: Box<dyn Rule + Send>,
    pub condition: Condition,
}

pub struct RuleComposite {
    pub rules: Vec<MultiRule>,
}

impl Rule for RuleComposite {
    fn get_priority(&self) -> i32 {
        self.rules
            .iter()
            .map(|rule| rule.rule.get_priority())
            .max()
            .unwrap_or_default()
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        for rule in &self.rules {
            let result = rule.rule.evaluate(galaxy, evaluation);
            if rule.condition.eval(result.len() as f32) {
                return vec![];
            }
        }
        vec![0]
    }
}
