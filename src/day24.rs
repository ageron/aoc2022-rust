use itertools::Itertools;
use pathfinding::directed::astar::astar;

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    start_x: i32,
    start_y: i32,
    dx: i32,
    dy: i32,
}

impl Blizzard {
    fn position(&self, width: i32, height: i32, time: i32) -> (i32, i32) {
        let x = (self.start_x + time * self.dx - 1).rem_euclid(width - 2) + 1;
        let y = (self.start_y + time * self.dy - 1).rem_euclid(height - 2) + 1;
        (x, y)
    }
}

fn no_blizzard_there(
    vertical_blizzards: &[Vec<Blizzard>],
    horizontal_blizzards: &[Vec<Blizzard>],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    time: i32,
) -> bool {
    vertical_blizzards[x as usize]
        .iter()
        .chain(horizontal_blizzards[y as usize].iter())
        .all(|blizzard| blizzard.position(width, height, time) != (x, y))
}

fn find_shortest_path(
    vertical_blizzards: &[Vec<Blizzard>],
    horizontal_blizzards: &[Vec<Blizzard>],
    width: i32,
    height: i32,
    entrance: (i32, i32),
    exit: (i32, i32),
    start_time: i32,
) -> i32 {
    let successors = |&(x, y, time): &(i32, i32, i32)| {
        [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)]
            .into_iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|&(nx, ny)| {
                (nx, ny) == entrance
                    || (nx, ny) == exit
                    || ((1..width - 1).contains(&nx)
                        && (1..height - 1).contains(&ny)
                        && no_blizzard_there(
                            vertical_blizzards,
                            horizontal_blizzards,
                            width,
                            height,
                            nx,
                            ny,
                            time + 1,
                        ))
            })
            .map(|(nx, ny)| ((nx, ny, time + 1), 1))
            .collect_vec()
    };
    let heuristic = |&(x, y, _): &(i32, i32, i32)| (x - exit.0).abs() + (y - exit.1).abs();
    let success = |&(x, y, _): &(i32, i32, i32)| (x, y) == exit;
    let (_, total_cost) = astar(
        &(entrance.0, entrance.1, start_time),
        successors,
        heuristic,
        success,
    )
    .unwrap();
    total_cost
}

pub fn run(input: &str) {
    let valley = input.lines().map(|line| line.as_bytes()).collect_vec();
    let (width, height) = (valley[0].len() as i32, valley.len() as i32);
    let entrance = (valley[0].iter().position(|&b| b == b'.').unwrap() as i32, 0);
    let exit = (
        valley[(height - 1) as usize]
            .iter()
            .position(|&b| b == b'.')
            .unwrap() as i32,
        height - 1,
    );
    let mut vertical_blizzards = vec![vec![]; width as usize];
    let mut horizontal_blizzards = vec![vec![]; height as usize];
    valley.into_iter().enumerate().for_each(|(start_y, row)| {
        row.iter().enumerate().for_each(|(start_x, &cell)| {
            let (start_x, start_y) = (start_x as i32, start_y as i32);
            let blizzard = match cell {
                b'^' => Some(Blizzard {
                    start_x,
                    start_y,
                    dx: 0,
                    dy: -1,
                }),
                b'v' => Some(Blizzard {
                    start_x,
                    start_y,
                    dx: 0,
                    dy: 1,
                }),
                b'<' => Some(Blizzard {
                    start_x,
                    start_y,
                    dx: -1,
                    dy: 0,
                }),
                b'>' => Some(Blizzard {
                    start_x,
                    start_y,
                    dx: 1,
                    dy: 0,
                }),
                _ => None,
            };
            if let Some(blizzard) = blizzard {
                if blizzard.dx == 0 {
                    vertical_blizzards[start_x as usize].push(blizzard)
                } else {
                    horizontal_blizzards[start_y as usize].push(blizzard)
                }
            }
        })
    });

    let min_minutes_to_exit = find_shortest_path(
        &vertical_blizzards,
        &horizontal_blizzards,
        width,
        height,
        entrance,
        exit,
        0,
    );
    println!("{min_minutes_to_exit}");

    let min_minutes_back = find_shortest_path(
        &vertical_blizzards,
        &horizontal_blizzards,
        width,
        height,
        exit,
        entrance,
        min_minutes_to_exit,
    );
    let min_minutes_to_exit_again = find_shortest_path(
        &vertical_blizzards,
        &horizontal_blizzards,
        width,
        height,
        entrance,
        exit,
        min_minutes_to_exit + min_minutes_back,
    );
    let min_total = min_minutes_to_exit + min_minutes_back + min_minutes_to_exit_again;
    println!("{min_total}");
}
