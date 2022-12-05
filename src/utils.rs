use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_ints(input: &str, signed: bool) -> Vec<i64> {
    lazy_static! {
        static ref REGEX_SIGNED: Regex = Regex::new(r"(-?\d+)").unwrap();
        static ref REGEX_UNSIGNED: Regex = Regex::new(r"(\d+)").unwrap();
    }
    let captures = if signed {
        REGEX_SIGNED.captures_iter(input)
    } else {
        REGEX_UNSIGNED.captures_iter(input)
    };
    captures.map(|cap| cap[1].parse().unwrap()).collect_vec()
}

pub fn parse_int_vecs(input: &str, signed: bool) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| parse_ints(line, signed))
        .collect_vec()
}
