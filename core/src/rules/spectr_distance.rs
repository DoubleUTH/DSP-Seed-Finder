use crate::data::{
    enums::SpectrType,
    rule::{Condition, Rule},
    star_planets::StarWithPlanets,
};
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
        14
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        let good_stars: Vec<&StarWithPlanets> = galaxy
            .stars
            .iter()
            .filter(|sp| sp.star.get_spectr() == &self.spectr)
            .collect();

        if good_stars.is_empty() {
            return result;
        }

        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            if evaluation.is_known(index) {
                continue;
            }
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
            if self.count_condition.eval(count as f32) {
                result.push(index)
            }
        }
        result
    }
}
