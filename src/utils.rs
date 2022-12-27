use itertools::Itertools;

pub fn parse_ints(input: &str, signed: bool) -> Vec<i64> {
    input
        .split(|c: char| !(c.is_ascii_digit() || (signed && c == '-')))
        .filter_map(|s| s.parse().ok())
        .collect()
}

pub fn parse_int_vecs(input: &str, signed: bool) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| parse_ints(line, signed))
        .collect_vec()
}
