use itertools::Itertools;
use pathfinding::directed::bfs::bfs;
use std::hash::Hash;

/// For part 2, using BFS independently for each low position is *very* slow.
/// Instead, it's best to run BFS just once: to do this, we can use a starting
/// position named AnyLowPosition, whose successors are all the low positions.
/// This makes the code more verbose since it needs to handle the enum cases in
/// a few places, but it's almost two orders of magnitude faster.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Pos {
    AnyLowPosition,
    Position(i32, i32),
}
use Pos::*;

fn shortest_distance(
    signal_map: &[Vec<u8>],
    low_positions: &[Pos], // only used in part 2
    start_pos: Pos,
    target_pos: Pos,
) -> u32 {
    let height = signal_map.len() as i32;
    let width = signal_map[0].len() as i32;
    let successors = |&pos: &Pos| match pos {
        AnyLowPosition => low_positions.to_vec(),
        Position(x, y) => {
            let max_signal: u8 = signal_map[y as usize][x as usize] + 1;
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .filter(|&(nx, ny)| {
                    nx >= 0
                        && ny >= 0
                        && nx < width
                        && ny < height
                        && (b'a'..=max_signal).contains(&signal_map[ny as usize][nx as usize])
                })
                .map(|(x, y)| Position(x, y))
                .collect_vec()
        }
    };
    let success = |&pos: &Pos| pos == target_pos;
    bfs(&start_pos, successors, success).unwrap().len() as u32 - 1
}

pub fn run(input: &str) {
    let mut start_pos: Option<Pos> = None;
    let mut target_pos: Option<Pos> = None;
    let mut low_positions: Vec<Pos> = vec![];
    let signal_map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, c)| {
                    let (x, y) = (x as i32, y as i32);
                    match c {
                        b'a' | b'S' => {
                            low_positions.push(Position(x, y));
                            if c == b'S' {
                                start_pos = Some(Position(x, y));
                            }
                            b'a'
                        }
                        b'E' => {
                            target_pos = Some(Position(x, y));
                            b'z'
                        }
                        c => c,
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let (start_pos, target_pos) = (start_pos.unwrap(), target_pos.unwrap());

    let length = shortest_distance(&signal_map, &low_positions, start_pos, target_pos);
    println!("{}", length);

    let length = shortest_distance(&signal_map, &low_positions, AnyLowPosition, target_pos) - 1;
    println!("{}", length);
}
