use hashbrown::HashSet;
use std::collections::VecDeque;
use itertools::Itertools;

fn is_neighbor(droplet1: &[i64], droplet2: &[i64]) -> bool {
    let mut distance = 0;
    for (coord1, coord2) in droplet1.iter().zip(droplet2.iter()) {
        distance += (coord1 - coord2).abs();
    }
    distance == 1
}

fn num_free_surfaces(droplets: &HashSet<[i64; 3]>) -> usize {
    let num_neighbors = droplets
        .iter()
        .combinations(2)
        .filter(|points| is_neighbor(points[0], points[1]))
        .count();
    droplets.len() * 6 - num_neighbors * 2
}

fn num_surfaces_outside(droplets: &HashSet<[i64; 3]>) -> usize {
    let max_coords = (0..3)
        .map(|axis| droplets.iter().map(|coords| coords[axis]).max().unwrap())
        .collect_vec();
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::from([[-1, -1, -1]]);
    let mut num_surfaces = 0;
    while let Some(node) = to_visit.pop_front() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node.clone());
        'next_neighbor: for offset in [
            [0, 0, -1],
            [0, 0, 1],
            [0, 1, 0],
            [0, -1, 0],
            [1, 0, 0],
            [-1, 0, 0],
        ] {
            let mut neighbor_node = node;
            for axis in 0..3 {
                neighbor_node[axis] += offset[axis];
                if neighbor_node[axis] < -1 || neighbor_node[axis] > max_coords[axis] + 1 {
                    continue 'next_neighbor;
                }
            }
            if droplets.contains(&neighbor_node) {
                num_surfaces += 1;
                continue;
            }
            to_visit.push_back(neighbor_node);
        }
    }
    num_surfaces
}

pub fn run(input: &str) {
    let droplets: HashSet<[i64; 3]> = input
        .lines()
        .map(|line| line
            .split(',')
            .map(|v| v.parse().unwrap()).collect_vec().try_into().unwrap()
        )
        .collect();

    let surface = num_free_surfaces(&droplets);
    println!("{}", surface);

    let surface = num_surfaces_outside(&droplets);
    println!("{}", surface);
}
