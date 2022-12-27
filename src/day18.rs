use hashbrown::HashSet;
use itertools::Itertools;

fn is_neighbor(droplet1: &[i64], droplet2: &[i64]) -> bool {
    let mut distance = 0;
    for (coord1, coord2) in droplet1.iter().zip(droplet2.iter()) {
        distance += (coord1 - coord2).abs();
    }
    distance == 1
}

fn num_free_surfaces(droplets: &HashSet<Vec<i64>>) -> usize {
    let num_neighbors = droplets
        .iter()
        .combinations(2)
        .filter(|points| is_neighbor(points[0], points[1]))
        .count();
    droplets.len() * 6 - num_neighbors * 2
}

fn num_surfaces_outside(droplets: &HashSet<Vec<i64>>) -> usize {
    let max_coords = (0..3)
        .map(|axis| droplets.iter().map(|coords| coords[axis]).max().unwrap())
        .collect_vec();
    let mut visited = HashSet::new();
    let mut to_visit = HashSet::from([vec![-1, -1, -1]]);
    let mut num_surfaces = 0;
    while !to_visit.is_empty() {
        let mut to_visit_next = HashSet::new();
        for node in &to_visit {
            for offset in [
                [0, 0, -1],
                [0, 0, 1],
                [0, 1, 0],
                [0, -1, 0],
                [1, 0, 0],
                [-1, 0, 0],
            ] {
                let neighbor_node = node
                    .iter()
                    .zip(offset.iter())
                    .map(|(&coord, &change)| coord + change)
                    .collect_vec();
                if (0..3).any(|axis| {
                    neighbor_node[axis] < -1 || neighbor_node[axis] > max_coords[axis] + 1
                }) {
                    continue;
                }
                if droplets.contains(&neighbor_node) {
                    num_surfaces += 1;
                    continue;
                }
                if visited.contains(&neighbor_node)
                    || to_visit.contains(&neighbor_node)
                    || to_visit_next.contains(&neighbor_node)
                {
                    continue;
                }
                to_visit_next.insert(neighbor_node);
            }
            visited.insert(node.clone());
        }
        to_visit = to_visit_next;
    }
    num_surfaces
}

pub fn run(input: &str) {
    let droplets: HashSet<Vec<i64>> = input
        .lines()
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect_vec())
        .collect();

    let surface = num_free_surfaces(&droplets);
    println!("{}", surface);

    let surface = num_surfaces_outside(&droplets);
    println!("{}", surface);
}
