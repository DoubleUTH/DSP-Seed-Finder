use crate::data::enums::StarType;
use crate::data::galaxy::Galaxy;
use crate::data::rule::Condition;
use crate::data::rule::Evaluation;
use crate::data::rule::Rule;
use crate::data::vector3::Vector3;
use crate::evaluate_safe;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleXDistance {
    pub condition: Condition,
    pub all: bool,
}

impl Rule for RuleXDistance {
    fn get_priority(&self) -> i32 {
        14
    }

    fn evaluate(&self, galaxy: &Galaxy, evaluation: &Evaluation) -> u64 {
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
            return 0;
        }

        evaluate_safe!(galaxy, evaluation, |sp| {
            let star = &sp.star;
            if self.all {
                x_stars
                    .iter()
                    .all(|p| self.condition.eval(star.position.distance_from(p) as f32))
            } else {
                x_stars
                    .iter()
                    .any(|p| self.condition.eval(star.position.distance_from(p) as f32))
            }
        })
    }
}
