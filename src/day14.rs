use itertools::Itertools;
use std::collections::HashSet;

type Cave = HashSet<(i32, i32)>;

fn make_cave(segments: &[Vec<(i32, i32)>]) -> Cave {
    let mut cave = HashSet::new();
    for segment in segments {
        let mut segment_iter = segment.iter();
        let &(mut x, mut y) = segment_iter.next().unwrap();
        cave.insert((x, y));
        for &(target_x, target_y) in segment_iter {
            let (dx, dy) = ((target_x - x).signum(), (target_y - y).signum());
            loop {
                x += dx;
                y += dy;
                cave.insert((x, y));
                if (x, y) == (target_x, target_y) {
                    break;
                }
            }
        }
    }
    cave
}

fn num_units_before_flow_out(cave: &mut Cave, with_floor: bool) -> usize {
    let max_y = cave.iter().map(|&(_, y)| y).max().unwrap();
    let source = (500, 0);
    for num_units in 1.. {
        let (mut x, mut y) = source;
        let mut is_at_rest = false;
        while y <= max_y && !is_at_rest {
            is_at_rest = true;
            for (nx, ny) in [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
                if !cave.contains(&(nx, ny)) {
                    x = nx;
                    y = ny;
                    is_at_rest = false;
                    break;
                }
            }
        }
        if is_at_rest && (x, y) == source {
            return num_units;
        }
        if with_floor || is_at_rest {
            cave.insert((x, y));
        } else {
            return num_units - 1;
        }
    }
    unreachable!()
}

pub fn run(input: &str) {
    let segments: Vec<Vec<(i32, i32)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|xy| {
                    xy.split(',')
                        .map(|val| val.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();
    let mut cave = make_cave(&segments);

    let num_units = num_units_before_flow_out(&mut cave.clone(), false);
    println!("{num_units}");

    let num_units = num_units_before_flow_out(&mut cave, true);
    println!("{num_units}");
}
