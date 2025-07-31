use crate::data::rule::Rule;

pub struct RuleOr {
    pub rules: Vec<Box<dyn Rule + Send>>,
}

impl Rule for RuleOr {
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
        let mut e = evaluation.clone();
        for rule in &self.rules {
            let result = rule.evaluate(galaxy, &e);
            e.accept_many(&result);
            if e.is_done() {
                return e.collect_known();
            }
        }
        e.collect_known()
    }
}
