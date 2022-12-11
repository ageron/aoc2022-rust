use regex::Regex;

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    val1: Option<u64>,
    is_add: bool, // alternatively: mul
    val2: Option<u64>,
    divisible_by: u64,
    monkey_if_true: usize,
    monkey_if_false: usize,
    num_inspections: u64,
}

fn monkey_business(monkeys: &[Monkey], num_rounds: usize, modulus: u64, divide_by_3: bool) -> u64 {
    let mut monkeys = monkeys.to_vec();
    for _ in 0..num_rounds {
        for index in 0..monkeys.len() {
            for item in std::mem::take(&mut monkeys[index].items) {
                monkeys[index].num_inspections += 1;
                let val1 = monkeys[index].val1.unwrap_or(item);
                let val2 = monkeys[index].val2.unwrap_or(item);
                let result = if monkeys[index].is_add {
                    val1 + val2
                } else {
                    val1 * val2
                };
                let result = result % modulus;
                let result = if divide_by_3 { result / 3 } else { result };
                let new_index = if result % monkeys[index].divisible_by == 0 {
                    monkeys[index].monkey_if_true
                } else {
                    monkeys[index].monkey_if_false
                };
                monkeys[new_index].items.push(result);
            }
        }
    }
    monkeys.sort_by_key(|monkey| std::cmp::Reverse(monkey.num_inspections));
    monkeys[0].num_inspections * monkeys[1].num_inspections
}

pub fn run(input: &str) {
    let regex = Regex::new(
        r"Monkey (\d+):
  Starting items: (.*?)
  Operation: new = (old|\d+) ([*+]) (old|\d+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)",
    )
    .unwrap();
    let monkeys: Vec<Monkey> = regex
        .captures_iter(input)
        .map(|cap| Monkey {
            items: cap[2]
                .split(", ")
                .map(|value_str| value_str.parse().unwrap())
                .collect(),
            val1: cap[3].parse().ok(),
            is_add: &cap[4] == "+",
            val2: cap[5].parse().ok(),
            divisible_by: cap[6].parse().unwrap(),
            monkey_if_true: cap[7].parse().unwrap(),
            monkey_if_false: cap[8].parse().unwrap(),
            num_inspections: 0,
        })
        .collect();

    let modulus = monkeys.iter().map(|monkey| monkey.divisible_by).product();

    let level = monkey_business(&monkeys, 20, modulus, true);
    println!("{}", level);

    let level = monkey_business(&monkeys, 10_000, modulus, false);
    println!("{}", level);
}
