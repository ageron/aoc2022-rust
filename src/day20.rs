use itertools::Itertools;

fn mix(encrypted_data: &[i64], decryption_key: i64, num_rounds: i64) -> i64 {
    let encrypted_data = encrypted_data
        .iter()
        .map(|&val| val * decryption_key)
        .collect_vec();
    let len = encrypted_data.len();
    // doubly linked list of indices
    let mut next = (1..len).chain(0..1).collect_vec();
    let mut prev = (len - 1..len).chain(0..len - 1).collect_vec();
    for _ in 0..num_rounds {
        for index in 0..len {
            // Replace value with an equivalent but shorter value, if possible
            // For example, if there are 11 elements in the list, then going
            // right by 19 steps is equivalent to going left by 1 step.
            // Uses len - 1 instead of len because the moving value must be ignored.
            let middle = ((len - 1) / 2) as i64;
            let value = (encrypted_data[index] + middle).rem_euclid((len - 1) as i64) - middle;
            if value == 0 {
                continue;
            }
            // delete index from linked list
            let old_prev = prev[index];
            let old_next = next[index];
            next[old_prev] = old_next;
            prev[old_next] = old_prev;
            // find new previous index
            let mut new_prev = old_prev;
            if value > 0 {
                for _ in 0..value {
                    new_prev = next[new_prev];
                }
            } else {
                for _ in value..0 {
                    new_prev = prev[new_prev];
                }
            }
            // insert index after new_prev
            let new_next = next[new_prev];
            next[new_prev] = index;
            next[index] = new_next;
            prev[new_next] = index;
            prev[index] = new_prev;
        }
    }
    let mut index = encrypted_data.iter().position(|&val| val == 0).unwrap();
    let mut coordinates_sum = 0;
    for after in 1..=3000 {
        index = next[index];
        if after % 1000 == 0 {
            coordinates_sum += encrypted_data[index];
        }
    }
    coordinates_sum
}

pub fn run(input: &str) {
    let encrypted_data: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();

    let coord_sum = mix(&encrypted_data, 1, 1);
    println!("{coord_sum}");

    let coord_sum = mix(&encrypted_data, 811589153, 10);
    println!("{coord_sum}");
}
