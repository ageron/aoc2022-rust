use itertools::Itertools;
use std::collections::HashSet;

fn end_index_of_window_without_duplicates(input: &[u8], length: usize) -> usize {
    input
        .windows(length)
        .enumerate()
        .find(|(_, window)| window.iter().copied().collect::<HashSet<u8>>().len() == length)
        .map(|(index, _)| index)
        .unwrap()
        + length
}

pub fn run(input: &str) {
    let input = input.bytes().collect_vec();
    let num_chars_processed = end_index_of_window_without_duplicates(&input, 4);
    println!("{}", num_chars_processed);
    let num_chars_processed = end_index_of_window_without_duplicates(&input, 14);
    println!("{}", num_chars_processed);
}
