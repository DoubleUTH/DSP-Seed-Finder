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
    let r1 = rand.next_f64();
    let mut tmp_drunk: Vec<Vector3> = vec![];
    tmp_poses.push(Vector3::zero());
    let min_drunk_num = 6;
    let max_drunk_num = 8;
    let drunk_num_range = (max_drunk_num - min_drunk_num) as f64;
    let drunk_num = (r1 * drunk_num_range + (min_drunk_num as f64)) as i32;
    // First try to place drunks, other stars are produced around them.
    // Apparently they are drunks tied to some utility poles.
    for _ in 0..drunk_num {
        for _ in 0..256 {
            let u = rand.next_f64() * 2.0 - 1.0;
            // Stars should not leave central plane too far
            let w = (rand.next_f64() * 2.0 - 1.0) * flatten;
            let v = rand.next_f64() * 2.0 - 1.0;
            let r2 = rand.next_f64();
            let d = u * u + w * w + v * v;
            if (1e-8..=1.0).contains(&d) {
                let distance = d.sqrt();
                let step_len_mult = (r2 * step_diff + min_dist) / distance;
                let pt = Vector3(u * step_len_mult, w * step_len_mult, v * step_len_mult);
                // Now pt is placed along (u, v, w) and (r2 * step_diff + min_dist) away
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
                    let u = rand.next_f64() * 2.0 - 1.0;
                    let w = (rand.next_f64() * 2.0 - 1.0) * flatten;
                    let v = rand.next_f64() * 2.0 - 1.0;
                    let r3 = rand.next_f64();
                    let d = u * u + w * w + v * v;
                    if (1e-8..=1.0).contains(&d) {
                        let distance = d.sqrt();
                        let step_len_mult = (r3 * step_diff + min_dist) / distance;
                        let new_pt = Vector3(
                            pt.0 + u * step_len_mult,
                            pt.1 + w * step_len_mult,
                            pt.2 + v * step_len_mult,
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

fn check_collision(tmp_poses: &[Vector3], pt: &Vector3, min_dist: f64) -> bool {
    let min_dist_sq = min_dist * min_dist;
    tmp_poses
        .iter()
        .any(|pt1| pt1.distance_sq_from(pt) < min_dist_sq)
}

fn generate_stars(game_desc: &GameDesc) -> Vec<StarWithPlanets<'_>> {
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

    let r1 = rand.next_f32();
    let r2 = rand.next_f32();
    let r3 = rand.next_f32();
    let r4 = rand.next_f32();
    let black_hole_num = (0.01 * (star_count as f64) + (r1 as f64) * 0.3).ceil() as usize;
    let neutro_star_num = (0.01 * (star_count as f64) + (r2 as f64) * 0.3).ceil() as usize;
    let white_dwarf_num = (0.016 * (star_count as f64) + (r3 as f64) * 0.4).ceil() as usize;
    let giant_star_num = (0.013 * (star_count as f64) + (r4 as f64) * 1.3).ceil() as usize;
    let black_hole_start = star_count - black_hole_num;
    let neutron_star_start = black_hole_start - neutro_star_num;
    let white_dwarf_start = neutron_star_start - white_dwarf_num;
    // Pick a giant star from main seq stars in each giant group
    let giant_group_num = (white_dwarf_start - 1) / giant_star_num;
    let giant_offset = giant_group_num / 2;

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
            } else if index == white_dwarf_start - 1 {
                SpectrType::O
            } else {
                SpectrType::X
            };
            let need_type = if index % giant_group_num == giant_offset {
                StarType::GiantStar
            } else if index >= black_hole_start {
                StarType::BlackHole
            } else if index >= neutron_star_start {
                StarType::NeutronStar
            } else if index >= white_dwarf_start {
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

pub fn create_galaxy(game_desc: &GameDesc) -> Galaxy<'_> {
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

    rule.evaluate(&galaxy, &evaluation)
}
