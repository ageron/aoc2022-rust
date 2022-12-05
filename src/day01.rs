use itertools::Itertools;

pub fn run(input: &str) {
    let calories = input
        .split("\n\n")
        .map(|s| s.lines().map(|line| line.parse::<i64>().unwrap()).sum())
        .sorted()
        .rev()
        .collect_vec();

    let max_calories = calories[0];
    println!("{max_calories}");

    let top_3_calories: i64 = calories[..3].iter().sum();
    println!("{top_3_calories}");
}
