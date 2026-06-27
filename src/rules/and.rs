use crate::data::galaxy::Galaxy;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;

pub struct RuleAnd {
    pub rules: Vec<Box<dyn Rule + Send + Sync>>,
}

impl Rule for RuleAnd {
    fn get_priority(&self) -> i32 {
        self.rules
            .iter()
            .map(|rule| rule.get_priority())
            .max()
            .unwrap_or_default()
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        let mut e = *evaluation;
        for rule in &self.rules {
            let result = rule.evaluate(galaxy, &e);
            e.reject_others(result);
            if e.is_done() {
                return e.collect_unknown();
            }
        }
        e.collect_unknown()
    }
}
