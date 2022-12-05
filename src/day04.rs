use itertools::Itertools;
use std::ops::RangeInclusive;
use super::utils::parse_int_vecs;

fn contains(range1: &RangeInclusive<i64>, range2: &RangeInclusive<i64>) -> bool {
    range1.start() <= range2.start() && range1.end() >= range2.end()
}

fn one_contains_the_other(ranges: &&(RangeInclusive<i64>, RangeInclusive<i64>)) -> bool {
    contains(&ranges.0, &ranges.1) || contains(&ranges.1, &ranges.0)
}

fn overlapping(ranges: &&(RangeInclusive<i64>, RangeInclusive<i64>)) -> bool {
    ranges.0.start() <= ranges.1.end() && ranges.0.end() >= ranges.1.start()
}

pub fn run(input: &str) {
    let ranges = parse_int_vecs(input, false)
        .iter()
        .map(|values| (values[0]..=values[1], values[2]..=values[3]))
        .collect_vec();
    let num_contain = ranges.iter().filter(one_contains_the_other).count();
    println!("{}", num_contain);
    let num_overlap = ranges.iter().filter(overlapping).count();
    println!("{}", num_overlap);
}
