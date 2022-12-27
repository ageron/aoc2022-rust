use hashbrown::HashMap;
use itertools::Itertools;
use num::{Rational64, Zero};

#[derive(Debug, Clone)]
enum MonkeyBusiness {
    Number(i64),
    Operation {
        left: String,
        op: char,
        right: String,
    },
    HumanVariable,
}
use MonkeyBusiness::*;

/// This function assumes that the provided equation simplifies to either an
/// integer, or to a linear equation ax + b, where x is the human variable, and
/// a and b are rational numbers. If the function runs into the = operation
/// which happens in part2 when evaluating "root", it solves the equation for x
/// and returns the solution as (0, solution).
fn evaluate(monkeys: &HashMap<String, MonkeyBusiness>, name: &str) -> (Rational64, Rational64) {
    let action = monkeys.get(name).unwrap();
    match action {
        Number(num) => (Rational64::new(0, 1), Rational64::new(*num, 1)),
        Operation { left, op, right } => {
            let (a, b) = evaluate(monkeys, left);
            let (c, d) = evaluate(monkeys, right);
            match *op {
                '+' => (a + c, b + d), // (ax + b) + (cx + d) = (a + c)x + (b + d)
                '-' => (a - c, b - d), // (ax + b) - (cx + d) = (a - c)x + (b - d)
                '*' => {
                    // (ax + b) * (cx + d) = acx² + (ad + bc)x + bd
                    assert!(a * c == Rational64::zero()); // assuming no x² term
                    ((a * d + b * c), b * d)
                }
                '/' => {
                    // (ax + b) / (cx + d) = a/d x + b/d  assuming c is 0
                    assert!(c == Rational64::zero());
                    (a / d, b / d)
                }
                '=' => {
                    // solve ax + b = cx + d => x = (d - b) / (a - c) assuming a ≠ c
                    assert!(a != c);
                    let solution = (d - b) / (a - c);
                    assert!(*solution.denom() == 1); // assuming the solution is an integer
                    (Rational64::zero(), solution)
                }
                _ => unreachable!(),
            }
        }
        HumanVariable => (Rational64::new(1, 1), Rational64::zero()), // x = 1 * x + 0
    }
}

pub fn run(input: &str) {
    let mut monkeys: HashMap<String, MonkeyBusiness> = input
        .lines()
        .map(|line| {
            let (name, action) = line.split(": ").collect_tuple().unwrap();
            let name = name.to_string();
            if let Ok(num) = action.parse::<i64>() {
                (name, Number(num))
            } else {
                let mut parts = action.split(' ');
                (
                    name,
                    Operation {
                        left: parts.next().unwrap().to_string(),
                        op: parts.next().unwrap().chars().next().unwrap(),
                        right: parts.next().unwrap().to_string(),
                    },
                )
            }
        })
        .collect();

    let (_, result) = evaluate(&monkeys, "root");
    println!("{}", result.to_integer());

    // make the tweaks for part 2
    monkeys.insert("humn".to_string(), HumanVariable);
    let root = monkeys.get_mut(&"root".to_string()).unwrap();
    if let Operation {
        left: _,
        op,
        right: _,
    } = root
    {
        *op = '=';
    } else {
        panic!("root should be an Operation");
    }

    let (_, result) = evaluate(&monkeys, "root");
    println!("{}", result);
}
