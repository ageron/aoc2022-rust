use hashbrown::HashSet;
use itertools::Itertools;
//6311
fn simulate(moves: &[(u8, i32)], num_knots: usize) -> usize {
    let mut rope: Vec<[i32; 2]> = vec![[0, 0]; num_knots];
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    for &(direction, distance) in moves {
        let (dx, dy) = match direction {
            b'U' => (0, -1),
            b'D' => (0, 1),
            b'L' => (-1, 0),
            b'R' => (1, 0),
            _ => unreachable!(),
        };
        for _ in 0..distance {
            rope[0][0] += dx;
            rope[0][1] += dy;
            for index in 1..num_knots {
                let dx = rope[index - 1][0] - rope[index][0];
                let dy = rope[index - 1][1] - rope[index][1];
                if dx.abs() > 1 || dy.abs() > 1 {
                    let catchup_dx = dx.signum();
                    let catchup_dy = dy.signum();
                    rope[index][0] += catchup_dx;
                    rope[index][1] += catchup_dy;
                    if index == num_knots - 1 {
                        visited.insert((rope[index][0], rope[index][1]));
                    }
                }
            }
        }
    }
    visited.len()
}

pub fn run(input: &str) {
    let moves: Vec<(u8, i32)> = input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split(' ').collect_tuple::<(&str, &str)>().unwrap();
            (
                direction.bytes().next().unwrap(),
                distance.parse::<i32>().unwrap(),
            )
        })
        .collect_vec();

    let num_visited = simulate(&moves, 2);
    println!("{}", num_visited);

    let num_visited = simulate(&moves, 10);
    println!("{}", num_visited);
}
