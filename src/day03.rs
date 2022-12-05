use itertools::Itertools;
use std::collections::HashSet;

fn priority(item: u8) -> u32 {
    match item {
        c if (b'a'..=b'z').contains(&c) => (c - b'a' + 1) as u32,
        c if (b'A'..=b'Z').contains(&c) => (c - b'A' + 27) as u32,
        _ => unreachable!(),
    }
}

fn find_common_item(item_groups: &[&[u8]]) -> u8 {
    item_groups
        .iter()
        // map to HashSets
        .map(|r| r.iter().copied().collect::<HashSet<u8>>())
        .into_iter()
        // reduce by intersecting
        .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<u8>>())
        .unwrap()
        // extract the unique common item
        .iter()
        .copied()
        .exactly_one()
        .unwrap()
}

fn get_score_part1(data: &[Vec<u8>]) -> u32 {
    data.iter()
        .map(|rucksack| {
            let middle = rucksack.len() / 2;
            let compartment1 = &rucksack[..middle];
            let compartment2 = &rucksack[middle..];
            let common_item = find_common_item(&[compartment1, compartment2]);
            priority(common_item)
        })
        .sum()
}

fn get_score_part2(data: &[Vec<u8>]) -> u32 {
    data.chunks(3)
        .map(|rucksacks| {
            let rucksacks = rucksacks.iter().map(|r| &r[..]).collect_vec();
            let common_item = find_common_item(&rucksacks);
            priority(common_item)
        })
        .sum()
}

pub fn run(input: &str) {
    let data = input
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();
    let score = get_score_part1(&data);
    println!("{}", score);
    let score = get_score_part2(&data);
    println!("{}", score);
}
