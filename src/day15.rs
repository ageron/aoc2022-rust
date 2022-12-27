use hashbrown::HashSet;

use super::utils::parse_int_vecs;
use itertools::Itertools;

#[derive(Copy, Clone)]
struct Sensor {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
}

impl Sensor {
    fn new(data: &[i64]) -> Self {
        Self {
            sx: data[0],
            sy: data[1],
            bx: data[2],
            by: data[3],
        }
    }
    fn radius(&self) -> i64 {
        (self.bx - self.sx).abs() + (self.by - self.sy).abs()
    }
    fn in_range(&self, x: i64, y: i64) -> bool {
        (self.sx - x).abs() + (self.sy - y).abs() <= self.radius()
    }
}

/// Merges overlapping or contiguous ranges. Each range is x_min..=x_max.
fn range_union(ranges: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut ranges = ranges.to_vec();
    ranges.sort();
    let mut union = vec![];
    let (mut union_x_min, mut union_x_max) = ranges[0];
    for (x_min, x_max) in ranges.into_iter().skip(1) {
        if x_min > union_x_max + 1 {
            union.push((union_x_min, union_x_max));
            (union_x_min, union_x_max) = (x_min, x_max);
        } else if x_max > union_x_max {
            union_x_max = x_max;
        }
    }
    union.push((union_x_min, union_x_max));
    union
}

/// Rather than check every possible location on the row, we consider the ranges
/// of sensor coverage, it's a bit more complex, but *much* faster.
fn count_impossible_locations_in_row(sensors: &[Sensor], y: i64) -> i64 {
    let impossible_ranges = sensors
        .iter()
        .map(|sensor| {
            let width_at_y = sensor.radius() - (sensor.sy - y).abs();
            let (mut min_x, mut max_x) = (sensor.sx - width_at_y, sensor.sx + width_at_y);
            if sensor.by == y {
                if sensor.bx == min_x {
                    min_x += 1
                } else {
                    max_x -= 1
                }
            }
            (min_x, max_x)
        })
        .filter(|&(min_x, max_x)| min_x <= max_x)
        .collect_vec();
    range_union(&impossible_ranges)
        .into_iter()
        .map(|(x_min, x_max)| x_max - x_min + 1)
        .filter(|&width| width > 0)
        .sum()
}

/// Iterating through all 4 million rows works fine, and it's simple enough, but
/// it runs in 1.3s in release mode (or about 20s in debug mode). That was fine
/// to get the star, but it's not very satisfactory, so I looked for a more
/// efficient solution.
/// The location we're looking for must be surrounded by sensor ranges. So the
/// code looks for all the diagonal lines (NW-to-SE, and SW-to-NE) that are just
/// one cell outside of the sensor ranges. The solution must be located at the
/// intersection of one of the SW-to-NE lines and one of the NW-to-SE lines.
/// Each NW-to-SE line is defined by y = -x + offset ± (radius + 1), and each
/// SW-to-NE line is defined by y = x + offset ± (radius + 1)
/// So the code starts by looking for all the possible offsets, in each
/// direction. I used HashSets to remove duplicates.
/// We can then narrow the search by considering only the NW-to-SE lines that
/// are located at the SW of one range and also at the NE of another range.
/// Similarly we only consider the SW-to-NE lines that are located at the NW of
/// one range and also at the SE of another.
/// Then we look at all the intersections of the remaining SW-to-NE and NW-to-SE
/// lines, and we keep only the location that is not in range of any sensor.
/// Caveat: this algorithm fails if the distress beacon is located on one of the
/// borders, since it can be surrounded by only two sensor ranges (or just one
/// if it's in a corner). But Eric Wastl is a nice guy, he wouldn't do that...
/// would he?
/// Anyway, should this happen, it's easy enough to also check for the
/// intersection between all diagonal lines and the borders. It's just a bit
/// boring, so I didn't bother.
/// In the end, this algorithm now runs in 220µs. That's about 6000x faster! 😃
fn find_tuning_frequency(sensors: &[Sensor]) -> i64 {
    let mut sw_offsets = HashSet::new();
    let mut nw_offsets = HashSet::new();
    let mut ne_offsets = HashSet::new();
    let mut se_offsets = HashSet::new();
    sensors.iter().for_each(|sensor| {
        let out_radius = sensor.radius() + 1;
        sw_offsets.insert(sensor.sy + sensor.sx - out_radius);
        nw_offsets.insert(sensor.sy - sensor.sx - out_radius);
        ne_offsets.insert(sensor.sy + sensor.sx + out_radius);
        se_offsets.insert(sensor.sy - sensor.sx + out_radius);
    });
    let sw_ne_offsets = sw_offsets.intersection(&ne_offsets).collect_vec();
    let nw_se_offsets = nw_offsets.intersection(&se_offsets).collect_vec();
    let (solution_x, solution_y) = sw_ne_offsets
        .into_iter()
        .cartesian_product(nw_se_offsets.into_iter())
        .filter(|&(&sw_ne_offset, &nw_se_offset)| (sw_ne_offset - nw_se_offset) % 2 == 0)
        .map(|(&sw_ne_offset, &nw_se_offset)| {
            let x = (sw_ne_offset - nw_se_offset) / 2;
            (x, x + nw_se_offset)
        })
        .filter(|&(x, y)| sensors.iter().all(|sensor| !sensor.in_range(x, y)))
        .exactly_one()
        .unwrap();
    let max = 4_000_000;
    solution_x * max + solution_y
}

pub fn run(input: &str) {
    let sensors: Vec<Sensor> = parse_int_vecs(input, true)
        .iter()
        .map(|data| Sensor::new(data))
        .collect_vec();

    let y = 2_000_000;
    let num_impossible = count_impossible_locations_in_row(&sensors, y);
    println!("{num_impossible}");

    let frequency = find_tuning_frequency(&sensors);
    println!("{frequency}");
}
