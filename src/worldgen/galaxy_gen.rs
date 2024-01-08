use super::name_gen::random_name;
use crate::data::enums::{SpectrType, StarType};
use crate::data::galaxy::Galaxy;
use crate::data::game_desc::GameDesc;
use crate::data::random::DspRandom;
use crate::data::rule::{Evaluaton, Rule};
use crate::data::star::Star;
use crate::data::star_planets::StarWithPlanets;
use crate::data::vector3::Vector3;
use std::rc::Rc;

fn generate_temp_poses(
    seed: i32,
    target_count: usize,
    iter_count: usize,
    min_dist: f64,
    min_step_len: f64,
    max_step_len: f64,
    flatten: f64,
) -> Vec<Vector3> {
    let mut tmp_poses: Vec<Vector3> = vec![];
    let actual_iter_count = iter_count.clamp(1, 16);
    random_poses(
        &mut tmp_poses,
        seed,
        target_count * actual_iter_count,
        min_dist,
        max_step_len - min_step_len,
        flatten,
    );

    for index in (0..tmp_poses.len()).rev() {
        if index % iter_count != 0 {
            tmp_poses.remove(index);
        }
        if tmp_poses.len() <= target_count {
            break;
        }
    }

    tmp_poses
}

fn random_poses(
    tmp_poses: &mut Vec<Vector3>,
    seed: i32,
    max_count: usize,
    min_dist: f64,
    step_diff: f64,
    flatten: f64,
) {
    let mut rand = DspRandom::new(seed);
    let num1 = rand.next_f64();
    let mut tmp_drunk: Vec<Vector3> = vec![];
    tmp_poses.push(Vector3::zero());
    let num2 = 6;
    let num3 = 8;
    let num4 = (num3 - num2) as f64;
    let num5 = (num1 * num4 + (num2 as f64)) as i32;
    for _ in 0..num5 {
        for _ in 0..256 {
            let num7 = rand.next_f64() * 2.0 - 1.0;
            let num8 = (rand.next_f64() * 2.0 - 1.0) * flatten;
            let num9 = rand.next_f64() * 2.0 - 1.0;
            let num10 = rand.next_f64();
            let d = num7 * num7 + num8 * num8 + num9 * num9;
            if d <= 1.0 && d >= 1e-8 {
                let num11 = d.sqrt();
                let num12 = (num10 * step_diff + min_dist) / num11;
                let pt = Vector3(num7 * num12, num8 * num12, num9 * num12);
                if !check_collision(tmp_poses, &pt, min_dist) {
                    tmp_drunk.push(pt.clone());
                    tmp_poses.push(pt);
                    if tmp_poses.len() >= max_count {
                        return;
                    }
                    break;
                }
            }
        }
    }
    for _ in 0..256 {
        for pt in tmp_drunk.iter_mut() {
            if rand.next_f64() <= 0.7 {
                for _ in 0..256 {
                    let num15 = rand.next_f64() * 2.0 - 1.0;
                    let num16 = (rand.next_f64() * 2.0 - 1.0) * flatten;
                    let num17 = rand.next_f64() * 2.0 - 1.0;
                    let num18 = rand.next_f64();
                    let d = num15 * num15 + num16 * num16 + num17 * num17;
                    if d <= 1.0 && d >= 1e-8 {
                        let num19 = d.sqrt();
                        let num20 = (num18 * step_diff + min_dist) / num19;
                        let new_pt = Vector3(
                            pt.0 + num15 * num20,
                            pt.1 + num16 * num20,
                            pt.2 + num17 * num20,
                        );
                        if !check_collision(tmp_poses, &new_pt, min_dist) {
                            *pt = new_pt.clone();
                            tmp_poses.push(new_pt);
                            if tmp_poses.len() >= max_count {
                                return;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn check_collision(tmp_poses: &Vec<Vector3>, pt: &Vector3, min_dist: f64) -> bool {
    let min_dist_sq = min_dist * min_dist;
    tmp_poses
        .iter()
        .any(|pt1| pt1.distance_sq_from(pt) < min_dist_sq)
}

fn generate_stars<'a>(game_desc: &'a GameDesc) -> Vec<StarWithPlanets<'a>> {
    let galaxy_seed = game_desc.seed;

    let mut rand = DspRandom::new(galaxy_seed);
    let tmp_poses = generate_temp_poses(
        rand.next_seed(),
        game_desc.star_count,
        4,
        2.0,
        2.3,
        3.5,
        0.18,
    );
    let star_count = tmp_poses.len();

    let num1 = rand.next_f32();
    let num2 = rand.next_f32();
    let num3 = rand.next_f32();
    let num4 = rand.next_f32();
    let num5 = (0.01 * (star_count as f64) + (num1 as f64) * 0.3).ceil() as usize;
    let num6 = (0.01 * (star_count as f64) + (num2 as f64) * 0.3).ceil() as usize;
    let num7 = (0.016 * (star_count as f64) + (num3 as f64) * 0.4).ceil() as usize;
    let num8 = (0.013 * (star_count as f64) + (num4 as f64) * 1.3).ceil() as usize;
    let num9 = star_count - num5;
    let num10 = num9 - num6;
    let num11 = num10 - num7;
    let num12 = (num11 - 1) / num8;
    let num13 = num12 / 2;

    let mut stars: Vec<StarWithPlanets> = vec![];

    for (index, position) in tmp_poses.into_iter().enumerate() {
        let seed = rand.next_seed();
        if index == 0 {
            stars.push(StarWithPlanets::new(Rc::new(Star::new(
                game_desc,
                0,
                seed,
                Vector3::zero(),
                StarType::MainSeqStar,
                &SpectrType::X,
            ))));
        } else {
            let need_spectr = if index == 3 {
                SpectrType::M
            } else if index == num11 - 1 {
                SpectrType::O
            } else {
                SpectrType::X
            };
            let need_type = if index % num12 == num13 {
                StarType::GiantStar
            } else if index >= num9 {
                StarType::BlackHole
            } else if index >= num10 {
                StarType::NeutronStar
            } else if index >= num11 {
                StarType::WhiteDwarf
            } else {
                StarType::MainSeqStar
            };
            stars.push(StarWithPlanets::new(Rc::new(Star::new(
                game_desc,
                index,
                seed,
                position,
                need_type,
                &need_spectr,
            ))));
        }
    }
    stars
}

pub fn create_galaxy<'a>(game_desc: &'a GameDesc) -> Galaxy<'a> {
    let mut stars = generate_stars(game_desc);
    let mut names: Vec<&str> = vec![];

    for sp in stars.iter_mut() {
        let name = random_name(sp.star.name_seed, &sp.star, names.iter());
        sp.name = name;
        names.push(&sp.name);
        sp.load_planets();
    }

    Galaxy {
        seed: game_desc.seed,
        stars,
    }
}

pub fn find_stars(game_desc: &GameDesc, rule: &mut Box<dyn Rule + Send>) -> Vec<usize> {
    let galaxy = Galaxy {
        seed: game_desc.seed,
        stars: generate_stars(game_desc),
    };

    let evaluation = Evaluaton::new(game_desc.star_count);
    let result = rule.evaluate(&galaxy, &evaluation);

    result
}
