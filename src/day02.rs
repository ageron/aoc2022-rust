use itertools::Itertools;

fn get_score(strategy: &[(u8, u8)], part1: bool) -> i32 {
    strategy
        .iter()
        .map(|&(elf, me)| {
            let me = if part1 { me } else { (me + elf + 2) % 3 };
            let outcome: u8 = (3 + elf - me) % 3;
            me as i32 + 1 + [3, 0, 6][outcome as usize]
        })
        .sum()
}

pub fn run(input: &str) {
    let strategy: Vec<(u8, u8)> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|v| v.bytes().next().unwrap())
                .collect_vec()
        })
        .map(|v| (v[0] - b'A', v[1] - b'X'))
        .collect_vec();

    let score = get_score(&strategy, true);
    println!("{}", score);
    let score = get_score(&strategy, false);
    println!("{}", score);
}
