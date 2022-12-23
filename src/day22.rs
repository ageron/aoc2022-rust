use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Action {
    MoveForward(i32),
    TurnLeft,
    TurnRight,
}
use Action::*;

fn offboard(board: &[Vec<u8>], x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return true;
    }
    let (x, y) = (x as usize, y as usize);
    if y >= board.len() {
        return true;
    }
    let row = &board[y];
    x >= row.len() || row[x] == b' '
}

fn get_password(board: &[Vec<u8>], actions: &[Action], is_cube: bool) -> i32 {
    let start_x = board[0].iter().position(|&cell| cell == b'.').unwrap() as i32;
    let (mut x, mut y) = (start_x, 0);
    let (mut dx, mut dy) = (1, 0);
    for action in actions {
        match action {
            TurnLeft => {
                (dx, dy) = (dy, -dx);
            }
            TurnRight => {
                (dx, dy) = (-dy, dx);
            }
            MoveForward(steps) => {
                for _ in 0..*steps {
                    let (mut nx, mut ny, mut ndx, mut ndy) = (x + dx, y + dy, dx, dy);
                    if offboard(board, nx, ny) {
                        if is_cube {
                            // This is the quick & dirty manual solution to the cube problem.
                            // It will only work if you have the same board shape as mine.
                            // Finding the 14 transition rules manually was the fastest
                            // way for me to solve this problem, but I'll probably revisit
                            // this later to find a more general & elegant solution.
                            // The 14 rules below correspond to the following movements:
                            //
                            //                         1234
                            //                         ↑↓↑↓
                            //                       5←FFRR→7
                            //                       6→FFRR←8
                            //                       9↙DD↙↑11
                            //                     10↓↗DD↗12
                            //                     6←LLBB→8
                            //                     5→LLBB←7
                            //                     1→UU↙↑13
                            //                     2←UU↗14
                            //                       ↑↓
                            //                       34
                            //
                            // F=Front, B=Back, R=Right, L=Left, D=Down, U=Up
                            (nx, ny, ndx, ndy) = match (nx, ny, dx, dy) {
                                (nx, -1, _, _) if (50..100).contains(&nx) => (0, 100 + nx, 1, 0), //  1
                                (-1, ny, _, _) if (150..200).contains(&ny) => (ny - 100, 0, 0, 1), //  2
                                (nx, -1, _, _) if (100..150).contains(&nx) => {
                                    (nx - 100, 199, 0, -1)
                                } //  3
                                (nx, 200, _, _) if (0..50).contains(&nx) => (nx + 100, 0, 0, 1), //  4
                                (49, ny, _, _) if (0..50).contains(&ny) => (0, 149 - ny, 1, 0), //  5
                                (-1, ny, _, _) if (100..150).contains(&ny) => (50, 149 - ny, 1, 0), //  6
                                (150, ny, _, _) if (0..50).contains(&ny) => (99, 149 - ny, -1, 0), //  7
                                (100, ny, _, _) if (100..150).contains(&ny) => {
                                    (149, 149 - ny, -1, 0)
                                } //  8
                                (49, ny, -1, 0) if (50..100).contains(&ny) => (ny - 50, 100, 0, 1), //  9
                                (nx, 99, 0, -1) if (0..50).contains(&nx) => (50, 50 + nx, 1, 0), // 10
                                (nx, 50, 0, 1) if (100..150).contains(&nx) => (99, nx - 50, -1, 0), // 11
                                (100, ny, 1, 0) if (50..100).contains(&ny) => (50 + ny, 49, 0, -1), // 12
                                (nx, 150, 0, 1) if (50..100).contains(&nx) => (49, nx + 100, -1, 0), // 13
                                (50, ny, 1, 0) if (150..200).contains(&ny) => {
                                    (ny - 100, 149, 0, -1)
                                } // 14
                                _ => (nx, ny, ndx, ndy),
                            }
                        } else {
                            loop {
                                (nx, ny) = (nx - dx, ny - dy);
                                if offboard(board, nx, ny) {
                                    (nx, ny) = (nx + dx, ny + dy);
                                    break;
                                }
                            }
                        }
                    }
                    if board[ny as usize][nx as usize] == b'#' {
                        break;
                    }
                    (x, y, dx, dy) = (nx, ny, ndx, ndy);
                }
            }
        }
    }
    let facing = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .iter()
        .position(|&pos| pos == (dx, dy))
        .unwrap() as i32;
    1000 * (y + 1) + 4 * (x + 1) + facing
}

pub fn run(input: &str) {
    let (board, path) = input.split("\n\n").collect_tuple().unwrap();
    let board: Vec<Vec<u8>> = board
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();
    let regex = Regex::new(r"(\d+|[LR])").unwrap();
    let actions = regex
        .captures_iter(path)
        .map(|cap| {
            if let Ok(steps) = cap[1].parse() {
                MoveForward(steps)
            } else {
                match &cap[1] {
                    "L" => TurnLeft,
                    "R" => TurnRight,
                    _ => unreachable!(),
                }
            }
        })
        .collect_vec();

    let password = get_password(&board, &actions, false);
    println!("{password}");

    let password = get_password(&board, &actions, true);
    println!("{password}");
}
