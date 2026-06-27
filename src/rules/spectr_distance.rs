use crate::data::enums::SpectrType;
use crate::data::galaxy::Galaxy;
use crate::data::rule::Condition;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::data::star_planets::StarWithPlanets;
use crate::evaluate_safe;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleSpectrDistance {
    pub spectr: SpectrType,
    pub distance_condition: Condition,
    pub count_condition: Condition,
}

impl Rule for RuleSpectrDistance {
    fn get_priority(&self) -> i32 {
        15
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
        let good_stars: Vec<&StarWithPlanets> = galaxy
            .stars
            .iter()
            .filter(|sp| sp.star.get_spectr() == self.spectr)
            .collect();

        if good_stars.is_empty() {
            return 0;
        }

        evaluate_safe!(galaxy, evaluation, |sp| {
            let star = &sp.star;
            let count = good_stars
                .iter()
                .filter(|sp2| {
                    sp2.star.index != star.index
                        && self
                            .distance_condition
                            .eval(star.position.distance_from(&sp2.star.position) as f32)
                })
                .count();
            self.count_condition.eval(count as f32)
        })
    }
}
