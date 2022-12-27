use super::utils::parse_int_vecs;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::max;

/// the number of resources required for each robot
type Blueprint = [[i64; 4]; 4];

fn ceil_div(a: i64, b: i64) -> i64 {
    (a + b - 1) / b
}

/// robots: number of robots of each type (ore, clay, obsidian, geode)
/// resources: number of resources of each type
fn max_opened_geodes(
    blueprint: Blueprint,
    robots: [i64; 4],
    resources: [i64; 4],
    time_left: i64,
    max_geodes: i64,
) -> i64 {
    let geodes_if_just_wait = resources[3] + time_left * robots[3];
    let mut max_geodes = max(max_geodes, geodes_if_just_wait);
    let upper_bound = resources[3] + time_left * robots[3] + time_left * (time_left - 1) / 2;
    if upper_bound <= max_geodes {
        return max_geodes;
    } // pruning
    for next_robot in (0..4).rev() {
        // priority: geode > obsidian > clay > ore
        let max_robots = match next_robot {
            0 => (0..4).map(|index| blueprint[index][0]).max().unwrap(), // max ore needed for any robot
            1 => blueprint[2][1], // clay required for obsidian
            2 => blueprint[3][2], // obsidian required for geode
            3 => i64::MAX,        // unlimited power geodes
            _ => unreachable!(),
        };
        if robots[next_robot] >= max_robots {
            continue;
        }
        let time_to_build = (0..4)
            .map(|index| {
                let have = resources[index];
                let need = blueprint[next_robot][index];
                let rate = robots[index];
                if need <= have {
                    1
                } else if rate == 0 {
                    time_left
                } else {
                    ceil_div(need - have, rate) + 1
                }
            })
            .max()
            .unwrap_or(time_left);
        let new_time_left = time_left - time_to_build;
        if new_time_left > 0 {
            let mut new_resources = resources;
            for index in 0..4 {
                new_resources[index] +=
                    robots[index] * time_to_build - blueprint[next_robot][index];
            }
            let mut new_robots = robots;
            new_robots[next_robot] += 1;
            let branch_max_geodes = max_opened_geodes(
                blueprint,
                new_robots,
                new_resources,
                new_time_left,
                max_geodes,
            );
            max_geodes = max(max_geodes, branch_max_geodes);
        }
    }
    max_geodes
}

fn sum_of_quality_levels(blueprints: &[Blueprint]) -> i64 {
    blueprints
        .par_iter()
        .enumerate()
        .map(|(index, &blueprint)| {
            (index as i64 + 1) * max_opened_geodes(blueprint, [1, 0, 0, 0], [0, 0, 0, 0], 24, 0)
        })
        .sum()
}

fn product_of_max_geodes(blueprints: &[Blueprint]) -> i64 {
    blueprints
        .par_iter()
        .map(|&blueprint| max_opened_geodes(blueprint, [1, 0, 0, 0], [0, 0, 0, 0], 32, 0))
        .product()
}

pub fn run(input: &str) {
    let blueprints: Vec<Blueprint> = parse_int_vecs(input, false)
        .into_iter()
        .map(|cost| {
            [
                [cost[1], 0, 0, 0],       // ore robot
                [cost[2], 0, 0, 0],       // clay robot
                [cost[3], cost[4], 0, 0], // obsidian robot
                [cost[5], 0, cost[6], 0], // geode robot
            ]
        })
        .collect_vec();

    let total_quality = sum_of_quality_levels(&blueprints);
    println!("{total_quality}");

    let geode_product = product_of_max_geodes(&blueprints[0..3]);
    println!("{geode_product}");
}
