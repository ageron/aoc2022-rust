use itertools::Itertools;
use super::utils::parse_int_vecs;

pub fn move_crates(stacks: &mut [Vec<u8>], moves: &[Vec<i64>], one_by_one: bool) -> String {
    for mv in moves {
        let (qty, from, to) = mv.iter().map(|&v| v as usize).collect_tuple().unwrap();
        let (from, to) = (from - 1, to - 1);
        let mid = stacks[from].len() - qty;
        let (_, crates) = stacks[from].split_at(mid);
        if one_by_one {
            stacks[to].append(&mut crates.iter().rev().copied().collect());
        } else {
            stacks[to].append(&mut crates.to_vec());
        }
        stacks[from].truncate(mid);
    }
    let top_crates = stacks
        .iter()
        .map(|stack| *stack.last().unwrap())
        .collect_vec();
    String::from_utf8(top_crates).unwrap()
}

pub fn run(input: &str) {
    let (stacks_str, moves_str) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks: Vec<Vec<u8>> = vec![];
    stacks_str.lines().rev().skip(1).for_each(|line| {
        line.bytes()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, byte)| (b'A'..=b'Z').contains(byte))
            .for_each(|(index, byte)| {
                if index >= stacks.len() {
                    stacks.push(vec![])
                }
                stacks[index].push(byte)
            });
    });
    let moves = parse_int_vecs(moves_str, false);

    let top_after_one_by_one_moves = move_crates(&mut stacks.clone(), &moves, true);
    println!("{}", top_after_one_by_one_moves);
    let top_after_batch_moves = move_crates(&mut stacks, &moves, false);
    println!("{}", top_after_batch_moves);
}
