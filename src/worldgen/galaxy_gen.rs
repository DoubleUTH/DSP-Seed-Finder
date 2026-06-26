use super::name_gen::random_name;
use crate::data::enums::{SpectrType, StarType};
use crate::data::galaxy::Galaxy;
use crate::data::game_desc::GameDesc;
use crate::data::random::DspRandom;
use crate::data::rule::{Evaluaton, Rule};
use crate::data::star::Star;
use crate::data::star_planets::StarWithPlanets;
use crate::data::vector3::Vector3;
use std::cell::Cell;
use std::rc::Rc;

const ITER_COUNT: usize = 4;
const MIN_DIST: f64 = 2.0;
const MIN_DIST_SQ: f64 = MIN_DIST * MIN_DIST;
const STEP_DIFF: f64 = 3.5 - 2.3;
const FLATTEN: f64 = 0.18;
const MIN_DRUNK_NUM: i32 = 6;
const MAX_DRUNK_NUM: i32 = 8;
const DRUNK_NUM_RANGE: f64 = (MAX_DRUNK_NUM - MIN_DRUNK_NUM) as f64;

fn generate_temp_poses(seed: i32, target_count: usize) -> Vec<Vector3> {
    let max_count = target_count * ITER_COUNT;
    let mut tmp_poses = Vec::with_capacity(max_count);
    random_poses(&mut tmp_poses, seed, max_count);

    for index in (0..tmp_poses.len()).rev() {
        if index % ITER_COUNT != 0 {
            tmp_poses.remove(index);
        }
        if tmp_poses.len() <= target_count {
            break;
        }
    }

    tmp_poses
}

fn random_poses(tmp_poses: &mut Vec<Vector3>, seed: i32, max_count: usize) {
    let mut rand = DspRandom::new(seed);
    let drunk_walk_count_rand = rand.next_f64();
    let mut tmp_drunk: Vec<Vector3> = Vec::with_capacity(max_count);
    tmp_poses.push(Vector3::zero());
    let drunk_num = (drunk_walk_count_rand * DRUNK_NUM_RANGE + (MIN_DRUNK_NUM as f64)) as i32;
    for _ in 0..drunk_num {
        for _ in 0..256 {
            let u = rand.next_f64() * 2.0 - 1.0;
            let w = (rand.next_f64() * 2.0 - 1.0) * FLATTEN;
            let v = rand.next_f64() * 2.0 - 1.0;
            let first_step_len_rand = rand.next_f64();
            let squared_length = u * u + w * w + v * v;
            if squared_length <= 1.0 && squared_length >= 1e-8 {
                let distance = squared_length.sqrt();
                let step_len_mult = (first_step_len_rand * STEP_DIFF + MIN_DIST) / distance;
                let pt = Vector3(u * step_len_mult, w * step_len_mult, v * step_len_mult);
                if !check_collision(tmp_poses, &pt) {
                    tmp_drunk.push(pt);
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
                    let u = rand.next_f64() * 2.0 - 1.0;
                    let w = (rand.next_f64() * 2.0 - 1.0) * FLATTEN;
                    let v = rand.next_f64() * 2.0 - 1.0;
                    let step_len_rand = rand.next_f64();
                    let squared_length2 = u * u + w * w + v * v;
                    if squared_length2 <= 1.0 && squared_length2 >= 1e-8 {
                        let distance = squared_length2.sqrt();
                        let step_len_mult = (step_len_rand * STEP_DIFF + MIN_DIST) / distance;
                        let new_pt = Vector3(
                            pt.0 + u * step_len_mult,
                            pt.1 + w * step_len_mult,
                            pt.2 + v * step_len_mult,
                        );
                        if !check_collision(tmp_poses, &new_pt) {
                            *pt = new_pt;
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

fn check_collision(tmp_poses: &Vec<Vector3>, pt: &Vector3) -> bool {
    tmp_poses
        .iter()
        .any(|existing_point| existing_point.distance_sq_from(pt) < MIN_DIST_SQ)
}

fn generate_stars<'a>(
    seed: i32,
    game_desc: &'a GameDesc,
    habitable_count: &'a Cell<i32>,
) -> Vec<StarWithPlanets<'a>> {
    let mut rand = DspRandom::new(seed);
    let tmp_poses = generate_temp_poses(rand.next_seed(), game_desc.star_count);
    let star_count = tmp_poses.len();

    let black_hole_count_rand = rand.next_f32();
    let neutron_star_count_rand = rand.next_f32();
    let white_dwarf_count_rand = rand.next_f32();
    let giant_star_count_rand = rand.next_f32();
    let black_hole_num = ((0.01 * (star_count as f64) + (black_hole_count_rand as f64) * 0.3)
        as f32)
        .ceil() as usize;
    let neutro_star_num = ((0.01 * (star_count as f64) + (neutron_star_count_rand as f64) * 0.3)
        as f32)
        .ceil() as usize;
    let white_dwarf_num = ((0.016 * (star_count as f64) + (white_dwarf_count_rand as f64) * 0.4)
        as f32)
        .ceil() as usize;
    let giant_star_num = ((0.013 * (star_count as f64) + (giant_star_count_rand as f64) * 1.4)
        as f32)
        .ceil() as usize;
    let black_hole_start = star_count - black_hole_num;
    let neutron_star_start = black_hole_start - neutro_star_num;
    let white_dwarf_start = neutron_star_start - white_dwarf_num;
    let giant_group_num = (white_dwarf_start - 1) / giant_star_num;
    let giant_offset = giant_group_num / 2;

    let mut stars: Vec<StarWithPlanets> = Vec::with_capacity(star_count);

    for (index, position) in tmp_poses.into_iter().enumerate() {
        let seed = rand.next_seed();
        if index == 0 {
            stars.push(StarWithPlanets::new(
                Rc::new(Star::new(
                    game_desc,
                    0,
                    seed,
                    Vector3::zero(),
                    StarType::MainSeqStar,
                    &SpectrType::X,
                )),
                game_desc,
                habitable_count,
            ));
        } else {
            let need_spectr = if index == 3 {
                SpectrType::M
            } else if index == white_dwarf_start - 1 {
                SpectrType::O
            } else {
                SpectrType::X
            };
            let need_type = if index >= black_hole_start {
                StarType::BlackHole
            } else if index >= neutron_star_start {
                StarType::NeutronStar
            } else if index >= white_dwarf_start {
                StarType::WhiteDwarf
            } else if index % giant_group_num == giant_offset {
                StarType::GiantStar
            } else {
                StarType::MainSeqStar
            };
            stars.push(StarWithPlanets::new(
                Rc::new(Star::new(
                    game_desc,
                    index,
                    seed,
                    position,
                    need_type,
                    &need_spectr,
                )),
                game_desc,
                habitable_count,
            ));
        }
    }
    stars
}

pub fn create_galaxy<'a>(
    seed: i32,
    game_desc: &'a GameDesc,
    habitable_count: &'a Cell<i32>,
) -> Galaxy<'a> {
    let mut stars = generate_stars(seed, game_desc, habitable_count);
    let mut names: Vec<&str> = Vec::with_capacity(game_desc.star_count);

    for sp in stars.iter_mut() {
        let name = random_name(sp.star.name_seed, &sp.star, names.iter());
        sp.name = name;
        names.push(&sp.name);
        sp.load_planets();
    }

    Galaxy { seed, stars }
}

pub fn find_stars(
    seed: i32,
    game_desc: &GameDesc,
    rule: &Box<dyn Rule + Send + Sync>,
) -> Vec<usize> {
    let habitable_count = Cell::new(0_i32);
    let galaxy = Galaxy {
        seed,
        stars: generate_stars(seed, game_desc, &habitable_count),
    };

    let evaluation = Evaluaton::new(game_desc.star_count);
    let result = rule.evaluate(&galaxy, &evaluation);

    result
}
