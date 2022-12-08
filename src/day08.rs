use itertools::Itertools;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

/// if distance ≥ 1, it's the distance to blocking tree
/// if distance ≤ 0, then -distance is distance to the border
fn distance_to_tree_or_border(forest: &[Vec<u8>], x: usize, y: usize, dx: i32, dy: i32) -> i32 {
    let height = forest[y][x];
    let mut x = x as i32;
    let mut y = y as i32;
    for step in 1.. {
        x += dx;
        y += dy;
        if x < 0 || y < 0 || x >= forest[0].len() as i32 || y >= forest.len() as i32 {
            return 1 - step;
        }
        if forest[y as usize][x as usize] >= height {
            return step;
        }
    }
    unreachable!()
}

fn is_visible_tree(forest: &[Vec<u8>], x: usize, y: usize) -> bool {
    DIRECTIONS
        .into_iter()
        .any(|(dx, dy)| distance_to_tree_or_border(forest, x, y, dx, dy) <= 0)
}

fn num_visible_trees(forest: &[Vec<u8>]) -> usize {
    (0..forest[0].len())
        .cartesian_product(0..forest.len())
        .filter(|&(x, y)| is_visible_tree(forest, x, y))
        .count()
}

fn scenic_score(forest: &[Vec<u8>], x: usize, y: usize) -> i32 {
    DIRECTIONS.into_iter().fold(1, |product, (dx, dy)| {
        product * distance_to_tree_or_border(forest, x, y, dx, dy).abs()
    })
}

fn max_scenic_score(forest: &[Vec<u8>]) -> i32 {
    (0..forest[0].len())
        .cartesian_product(0..forest.len())
        .map(|(x, y)| scenic_score(forest, x, y))
        .max()
        .unwrap()
}

pub fn run(input: &str) {
    let forest: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|c| c - b'0').collect_vec())
        .collect_vec();

    let num = num_visible_trees(&forest);
    println!("{}", num);

    let score = max_scenic_score(&forest);
    println!("{}", score);
}
