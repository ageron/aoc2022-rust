use std::env;
use std::fs;
use std::time::Instant;

fn elapsed_since(start_time: &Instant) -> String {
    let elapsed = start_time.elapsed().as_micros();
    if elapsed >= 1_000_000 {
        let elapsed = elapsed as f64 / 1_000_000.0;
        format!("{elapsed:.1}s")
    } else if elapsed >= 1000 {
        let elapsed = elapsed as f64 / 1000.0;
        format!("{elapsed:.1}ms")
    } else {
        format!("{elapsed}µs")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<_> = match args.len() {
        1 => (1..=16).collect(),
        _ => args.iter().skip(1).map(|d| d.parse().unwrap()).collect(),
    };
    let global_start_time = Instant::now();
    for day in &days {
        let start_time = Instant::now();
        println!("Day {}:", day);
        let path = format!("./data/day{:02}.txt", day);
        let input = fs::read_to_string(&path);
        if let Ok(input) = input {
            let input = input.trim_end();
            let day_func = match day {
                1 => aoc2022::day01::run,
                2 => aoc2022::day02::run,
                3 => aoc2022::day03::run,
                4 => aoc2022::day04::run,
                5 => aoc2022::day05::run,
                6 => aoc2022::day06::run,
                7 => aoc2022::day07::run,
                8 => aoc2022::day08::run,
                9 => aoc2022::day09::run,
                10 => aoc2022::day10::run,
                11 => aoc2022::day11::run,
                12 => aoc2022::day12::run,
                13 => aoc2022::day13::run,
                14 => aoc2022::day14::run,
                15 => aoc2022::day15::run,
                16 => aoc2022::day16::run,
                _ => unreachable!(),
            };
            day_func(input);
            println!("Time: {}", elapsed_since(&start_time));
        } else {
            println!("ERROR: no data");
        }
        println!();
    }
    if days.len() > 1 {
        println!("TOTAL TIME: {}", elapsed_since(&global_start_time));
    }
}
