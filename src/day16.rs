use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Valve {
    index: usize,
    flow_rate: i32,
    distance: Vec<i32>, // shortest distance to each valve
    is_open: bool,
}

/// Floyd–Warshall algorithm to find all pair-wise distances between valves
/// See https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
fn find_all_distances(valve_tunnels: &[Vec<usize>]) -> Vec<Vec<i32>> {
    let len = valve_tunnels.len();
    let mut distance = vec![vec![len as i32; len]; len]; // len × len matrix
    for (from_index, to_indices) in valve_tunnels.iter().enumerate() {
        distance[from_index][from_index] = 0;
        for &to_index in to_indices {
            distance[from_index][to_index] = 1;
        }
    }
    for ((k, i), j) in (0..len).cartesian_product(0..len).cartesian_product(0..len) {
        distance[i][j] = min(distance[i][j], distance[i][k] + distance[k][j]);
    }
    distance
}

fn max_pressure_release(
    valves: &mut [Valve],
    pressure: i32,
    mut best_so_far: i32,
    locations: &mut Vec<usize>,
    next_decision_times: &mut Vec<i32>,
) -> i32 {
    let (time_left, actor_index) = next_decision_times
        .iter()
        .enumerate()
        .map(|(index, &time)| (time, index))
        .max()
        .unwrap();
    let location = locations[actor_index];

    // prioritize the next valves based on the maximum pressure we can release from them
    let prioritized_valves = valves
        .iter()
        .filter(|valve| valve.flow_rate > 0 && !valve.is_open)
        .map(|valve| (valve, valves[location].distance[valve.index]))
        .filter(|(_, distance)| time_left > distance + 1)
        .map(|(valve, distance)| {
            (
                (time_left - distance - 1) * valve.flow_rate,
                distance,
                valve.index,
            )
        })
        .sorted()
        .rev()
        .collect_vec();

    // prune branch if its upper bound is lower than the best we found so far
    let upper_bound = pressure
        + valves
            .iter()
            .filter(|valve| valve.flow_rate > 0 && !valve.is_open)
            .map(|valve| {
                locations
                    .iter()
                    .zip(next_decision_times.iter())
                    .map(|(&loc, &time)| {
                        let dist = valves[loc].distance[valve.index];
                        max(0, time - dist - 1) * valve.flow_rate
                    })
                    .max()
                    .unwrap_or(0)
            })
            .sum::<i32>();
    if upper_bound <= best_so_far {
        return pressure;
    }

    prioritized_valves
        .into_iter()
        .map(|(releasable_pressure, distance, to_index)| {
            valves[to_index].is_open = true;
            locations[actor_index] = to_index;
            next_decision_times[actor_index] = time_left - distance - 1;
            let new_pressure = pressure + releasable_pressure;
            best_so_far = max(best_so_far, new_pressure);
            let branch_pressure = max_pressure_release(
                valves,
                new_pressure,
                best_so_far,
                locations,
                next_decision_times,
            );
            best_so_far = max(branch_pressure, best_so_far);
            // revert changes
            valves[to_index].is_open = false;
            locations[actor_index] = location;
            next_decision_times[actor_index] = time_left;
            branch_pressure
        })
        .max()
        .unwrap_or(pressure)
}

pub fn run(input: &str) {
    // parse data
    let regex =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z ,]+)")
            .unwrap();
    let mut name_to_index: HashMap<String, usize> = HashMap::new();
    let mut tunnels = vec![];
    let mut valves = regex
        .captures_iter(input)
        .enumerate()
        .map(|(index, cap)| {
            name_to_index.insert(cap[1].to_string(), index);
            tunnels.push(cap[3].to_string());
            Valve {
                index,
                flow_rate: cap[2].parse::<i32>().unwrap(),
                distance: vec![],
                is_open: false,
            }
        })
        .collect_vec();

    // convert valve names to indices and find all pair-wise distances
    let valve_tunnels = tunnels
        .into_iter()
        .map(|tunnels| {
            tunnels
                .split(", ")
                .into_iter()
                .map(|name| *name_to_index.get(name).unwrap())
                .collect_vec()
        })
        .collect_vec();
    find_all_distances(&valve_tunnels)
        .into_iter()
        .enumerate()
        .for_each(|(index, distance)| {
            valves[index].distance = distance;
        });
    let &start_index = name_to_index.get("AA").unwrap();
    let pressure = max_pressure_release(&mut valves, 0, 30, &mut vec![start_index], &mut vec![30]);
    println!("{pressure}");

    let pressure = max_pressure_release(
        &mut valves,
        0,
        26,
        &mut vec![start_index, start_index],
        &mut vec![26, 26],
    );
    println!("{pressure}");
}
