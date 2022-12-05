use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<_> = match args.len() {
        1 => (1..=5).collect(),
        _ => args.iter().skip(1).map(|d| d.parse().unwrap()).collect(),
    };
    for day in days {
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
                _ => unreachable!(),
            };
            let start_time = Instant::now();
            day_func(input);
            let elapsed = start_time.elapsed().as_millis();
            println!("{}ms", elapsed);
        } else {
            println!("ERROR: no data");
        }
    }
}
