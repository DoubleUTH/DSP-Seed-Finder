use crate::data::galaxy::Galaxy;
use crate::data::rule::Condition;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;

pub struct RuleComposite {
    pub rule: Box<dyn Rule + Send + Sync>,
    pub condition: Condition,
}

impl Rule for RuleComposite {
    fn get_priority(&self) -> i32 {
        self.rule.get_priority()
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        let result = self.rule.evaluate(galaxy, evaluation);
        if self.condition.eval(result.count_ones() as f32) {
            1
        } else {
            0
        }
    }
}

pub struct RuleCompositeAnd {
    pub rules: Vec<Box<dyn Rule + Send + Sync>>,
}

impl Rule for RuleCompositeAnd {
    fn get_priority(&self) -> i32 {
        self.rules
            .iter()
            .map(|rule| rule.get_priority())
            .max()
            .unwrap_or_default()
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        for rule in &self.rules {
            let result = rule.evaluate(galaxy, evaluation);
            if result == 0 {
                return 0;
            }
        }
        1
    }
}

pub struct RuleCompositeOr {
    pub rules: Vec<Box<dyn Rule + Send + Sync>>,
}

impl Rule for RuleCompositeOr {
    fn get_priority(&self) -> i32 {
        self.rules
            .iter()
            .map(|rule| rule.get_priority())
            .max()
            .unwrap_or_default()
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        for rule in &self.rules {
            let result = rule.evaluate(galaxy, evaluation);
            if result != 0 {
                return 1;
            }
        }
        0
    }
}
