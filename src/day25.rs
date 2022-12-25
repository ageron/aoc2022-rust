const SNAFU_DIGITS: [char; 5] = ['=', '-', '0', '1', '2'];

fn parse_snafu(s: &str) -> i64 {
    let mut value = 0;
    for c in s.chars() {
        value *= 5;
        value += match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        };
    }
    value
}

fn to_snafu(value: i64) -> String {
    let mut value_up = value.abs();
    let mut value_down = value_up;
    let mut power = 1;
    loop {
        value_up += 2 * power;
        value_down -= 2 * power;
        if value_down <= 0 {
            break;
        }
        power *= 5;
    }
    let mut reversed_snafu = vec![];
    while value_up != 0 {
        let index = (value_up % 5) as usize;
        let index = if value > 0 { index } else { 4 - index };
        reversed_snafu.push(SNAFU_DIGITS[index]);
        value_up /= 5;
    }
    reversed_snafu.iter().rev().collect()
}

pub fn run(input: &str) {
    let total_snafus = input.lines().map(parse_snafu).sum();
    println!("{}", to_snafu(total_snafus));
}
