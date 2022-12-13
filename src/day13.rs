use itertools::Itertools;
use serde::Deserialize;
use serde_json::from_str;
use std::cmp::{Ord, Ordering};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
enum Packet {
    Integer(i64),
    List(Vec<Packet>),
}
use Packet::*;

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        let list1 = match self {
            Integer(num1) => match other {
                Integer(num2) => {
                    return num1.cmp(num2);
                }
                List(_) => {
                    vec![Integer(*num1)]
                }
            },
            List(vals1) => vals1.clone(),
        };
        let list2 = match other {
            Integer(num2) => {
                vec![Integer(*num2)]
            }
            List(vals2) => vals2.clone(),
        };
        for (packet1, packet2) in list1.iter().zip(list2.iter()) {
            let compare = packet1.cmp(packet2);
            match compare {
                Ordering::Less | Ordering::Greater => {
                    return compare;
                }
                _ => {}
            }
        }
        list1.len().cmp(&list2.len())
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn sum_good_indices(packets: &[Packet]) -> usize {
    packets
        .chunks(2)
        .map(|pair| pair[0].cmp(&pair[1]))
        .enumerate()
        .filter(|&(_, compare)| compare == Ordering::Less)
        .map(|(index, _)| index + 1)
        .sum()
}

fn decoder_key(packets: &[Packet]) -> usize {
    let mut packets = packets.to_vec();
    let divider_packet1: Packet = from_str("[[2]]").unwrap();
    let divider_packet2: Packet = from_str("[[6]]").unwrap();
    packets.push(divider_packet1.clone());
    packets.push(divider_packet2.clone());
    packets.sort();
    [divider_packet1, divider_packet2]
        .iter()
        .map(|packet| packets.iter().position(|val| val == packet).unwrap() + 1)
        .product()
}

pub fn run(input: &str) {
    let packets: Vec<Packet> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| from_str(line).unwrap())
        .collect_vec();

    let sum = sum_good_indices(&packets);
    println!("{}", sum);

    let key = decoder_key(&packets);
    println!("{}", key);
}
