use super::name_gen::random_name;
use super::planet_gen::{generate_gases, generate_veins, set_planet_theme};
use super::random::DspRandom;
use super::star_gen::{create_birth_star, create_star, create_star_planets};
use crate::data::enums::{PlanetType, SpectrType, StarType};
use crate::data::galaxy::Galaxy;
use crate::data::game_desc::GameDesc;
use crate::data::planet::Planet;
use crate::data::rule::Rule;
use crate::data::star::Star;
use crate::data::vector3::Vector3;

fn generate_temp_poses(
    seed: i32,
    target_count: i32,
    iter_count: i32,
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
        (target_count * actual_iter_count) as usize,
        min_dist,
        max_step_len - min_step_len,
        flatten,
    );

    for index in (0..tmp_poses.len()).rev() {
        if (index as i32) % iter_count != 0 {
            tmp_poses.remove(index);
        }
        if (tmp_poses.len() as i32) <= target_count {
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

fn generate_stars(game_desc: &GameDesc) -> impl Iterator<Item = Star> {
    let galaxy_seed = game_desc.seed;

    let mut rand = DspRandom::new(galaxy_seed);
    let tmp_poses = generate_temp_poses(rand.next_seed(), game_desc.star_count, 4, 2.0, 2.3, 3.5, 0.18);
    let star_count = tmp_poses.len() as i32;

    let num1 = rand.next_f32();
    let num2 = rand.next_f32();
    let num3 = rand.next_f32();
    let num4 = rand.next_f32();
    let num5 = (0.00999999977648258 * (star_count as f64) + (num1 as f64) * 0.300000011920929)
        .ceil() as i32;
    let num6 = (0.00999999977648258 * (star_count as f64) + (num2 as f64) * 0.300000011920929)
        .ceil() as i32;
    let num7 = (0.0160000007599592 * (star_count as f64) + (num3 as f64) * 0.400000005960464).ceil()
        as i32;
    let num8 =
        (0.0130000002682209 * (star_count as f64) + (num4 as f64) * 0.39999997615814).ceil() as i32;
    let num9 = star_count - num5;
    let num10 = num9 - num6;
    let num11 = num10 - num7;
    let num12 = (num11 - 1) / num8;
    let num13 = num12 / 2;

    (0..star_count).map(move |index| {
        let seed = rand.next_seed();
        if index == 0 {
            create_birth_star(seed)
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
            create_star(
                star_count,
                &tmp_poses[index as usize],
                index + 1,
                seed,
                need_type,
                need_spectr,
            )
        }
    })
}

fn sum_veins(star: &mut Star, planets: &Vec<Planet>) {
    for planet in planets {
        for vein in &planet.veins {
            let avg_patches = ((vein.min_patch + vein.max_patch) as f32) * ((vein.min_group + vein.max_group) as f32) / 4.0;
            let avg_amount = ((vein.min_amount + vein.max_amount) as f32) * avg_patches / 2.0;
            if let Some(x) = star.vein_patch.get_mut(&vein.vein_type) {
                *x += avg_patches;
            } else {
                star.vein_patch.insert(vein.vein_type.clone(), avg_patches);
            }
            if let Some(x) = star.vein_amount.get_mut(&vein.vein_type) {
                *x += avg_amount;
            } else {
                star.vein_amount.insert(vein.vein_type.clone(), avg_amount);
            }
        }
    }
}

pub fn create_galaxy(game_desc: &GameDesc) -> Galaxy {
    let mut galaxy = Galaxy::new();
    let mut habitable_count = 0;

    for mut star in generate_stars(game_desc) {
        star.name = random_name(star.name_seed, &star, galaxy.stars.iter().map(|s| &s.name));
        let mut planets = create_star_planets(&star, game_desc.star_count, &mut habitable_count);
        let mut used_theme_ids: Vec<i32> = vec![];
        let is_birth_star = star.index == 0;
        for planet in &mut planets {
            set_planet_theme(planet, is_birth_star, &mut used_theme_ids);
            if planet.planet_type == PlanetType::Gas {
                generate_gases(planet, &star, game_desc);
            } else {
                generate_veins(planet, &star, game_desc);
            }
        }
        sum_veins(&mut star, &planets);
        star.planets = planets;
        galaxy.stars.push(star);
    }

    galaxy.seed = game_desc.seed;

    return galaxy;
}

pub fn find_stars(game_desc: &GameDesc, rule: &mut Box<dyn Rule + Send>) -> Galaxy {
    let mut habitable_count = 0;
    let mut stars: Vec<Star> = vec![];
    let mut names: Vec<String> = vec![];

    for mut star in generate_stars(game_desc) {
        rule.reset();
        let name = random_name(star.name_seed, &star, names.iter());
        names.push(name);
        if rule.on_star_created(&star) == Some(false) {
            continue;
        }
        let mut planets = create_star_planets(&star, game_desc.star_count, &mut habitable_count);
        if !rule.is_evaluated() && rule.on_planets_created(&star, &planets) == Some(false) {
            continue;
        }
        let mut used_theme_ids: Vec<i32> = vec![];
        let is_birth_star = star.index == 0;
        for planet in &mut planets {
            set_planet_theme(planet, is_birth_star, &mut used_theme_ids);
        }
        if !rule.is_evaluated() && rule.on_planets_themed(&star, &planets) == Some(false) {
            continue;
        }
        for planet in &mut planets {
            if planet.planet_type == PlanetType::Gas {
                generate_gases(planet, &star, game_desc);
            } else {
                generate_veins(planet, &star, game_desc);
            }
        }
        sum_veins(&mut star, &planets);
        if !rule.is_evaluated() && rule.on_veins_generated(&star, &planets) == Some(false) {
            continue;
        }
        star.planets = planets;
        star.name = names.last().unwrap().clone();
        stars.push(star);
    }

    Galaxy { seed: game_desc.seed, stars }
}
