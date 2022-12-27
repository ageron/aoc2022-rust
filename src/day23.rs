use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn simulate_round(
    positions: &HashSet<(i32, i32)>,
    order_index: usize,
) -> (HashSet<(i32, i32)>, bool) {
    let mut proposed_positions: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut proposed_position_counts: HashMap<(i32, i32), i32> = HashMap::new();
    let mut is_stable_round = true;
    let directions = [
        [(-1, -1), (0, -1), (1, -1)], // north
        [(-1, 1), (0, 1), (1, 1)],    // south
        [(-1, -1), (-1, 0), (-1, 1)], // west
        [(1, -1), (1, 0), (1, 1)],    // east
    ];
    // first half of the round
    'next_elf: for &(x, y) in positions {
        let num_neighbor_elves = (x - 1..=x + 1)
            .cartesian_product(y - 1..=y + 1)
            .filter(|pos| positions.contains(pos))
            .count()
            - 1;
        if num_neighbor_elves == 0 {
            continue;
        }
        for direction_index in 0..4 {
            let direction_index = (order_index + direction_index) % 4;
            if directions[direction_index]
                .iter()
                .any(|&(dx, dy)| positions.contains(&(x + dx, y + dy)))
            {
                continue;
            }
            let (dx, dy) = directions[direction_index][1];
            let (proposed_x, proposed_y) = (x + dx, y + dy);
            proposed_positions.insert((x, y), (proposed_x, proposed_y));
            *proposed_position_counts
                .entry((proposed_x, proposed_y))
                .or_insert(0) += 1;
            continue 'next_elf;
        }
    }
    // second half of the round
    let single_vote_destinations: HashSet<(i32, i32)> = proposed_position_counts
        .iter()
        .filter(|&(_, num)| *num == 1)
        .map(|(&(x, y), _)| (x, y))
        .collect();
    let new_positions = positions
        .iter()
        .map(|old_pos| {
            let proposed = proposed_positions.get(old_pos);
            if let Some(proposed) = proposed {
                if single_vote_destinations.contains(proposed) {
                    is_stable_round = false;
                    *proposed
                } else {
                    *old_pos
                }
            } else {
                *old_pos
            }
        })
        .collect();
    (new_positions, is_stable_round)
}

fn num_empty_ground_tiles(positions: &HashSet<(i32, i32)>, num_rounds: usize) -> usize {
    let mut positions = positions.clone();
    for round in 0..num_rounds {
        (positions, _) = simulate_round(&positions, round);
    }
    let (min_x, max_x) = positions
        .iter()
        .map(|&(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = positions
        .iter()
        .map(|&(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();
    (min_x..=max_x)
        .cartesian_product(min_y..=max_y)
        .filter(|pos| !positions.contains(pos))
        .count()
}

fn first_stable_round(positions: &HashSet<(i32, i32)>) -> usize {
    let mut positions = positions.clone();
    for round in 0.. {
        let (new_positions, is_stable_round) = simulate_round(&positions, round);
        if is_stable_round {
            return round + 1;
        }
        positions = new_positions;
    }
    unreachable!()
}

pub fn run(input: &str) {
    let positions: HashSet<(i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();

    let num_tiles = num_empty_ground_tiles(&positions, 10);
    println!("{num_tiles}");

    let round_index = first_stable_round(&positions);
    println!("{round_index}");
}
