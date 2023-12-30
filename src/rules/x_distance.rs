use crate::data::{
    enums::StarType,
    rule::{Condition, Rule},
    vector3::Vector3,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleXDistance {
    pub condition: Condition,
}

impl Rule for RuleXDistance {
    fn get_priority(&self) -> i32 {
        13
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        let x_stars: Vec<&Vector3> = galaxy
            .stars
            .iter()
            .filter(|sp| {
                sp.star.star_type == StarType::BlackHole
                    || sp.star.star_type == StarType::NeutronStar
            })
            .map(|sp| &sp.star.position)
            .collect();

        if x_stars.is_empty() {
            return result;
        }

        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            if evaluation.is_known(index) {
                continue;
            }
            let star = &sp.star;
            let min_distance = x_stars
                .iter()
                .map(|p| star.position.distance_sq_from(p))
                .reduce(f64::min)
                .unwrap()
                .sqrt();
            if self.condition.eval(min_distance as f32) {
                result.push(index)
            }
        }
        result
    }
}
