use hashbrown::HashMap;
use itertools::Itertools;
use std::cmp::max;

const WIDTH: usize = 7;
const MAX_HEIGHT: usize = 10_000;
const NUM_TOP_ROWS: i64 = 10; // number of top rows to look at to detect repetion

fn drop_rock(
    jets: &[i64],
    shape: &[Vec<bool>],
    grid: &mut [Vec<bool>],
    time: &mut i64,
    height: &mut i64,
) {
    let shape_height = shape.len() as i64;
    let shape_width = shape[0].len() as i64;
    let mut x = 2;
    let mut y = *height + 3;
    'falling: loop {
        let jet = jets[*time as usize % jets.len()];
        *time += 1;
        'slide_and_drop: for (dx, dy) in [(jet, 0), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;
            if ny < 0 && dy == -1 {
                break 'falling;
            }
            if nx < 0 || nx + shape_width > 7 {
                continue;
            }
            for (cell_y, row) in shape.iter().enumerate() {
                for (cell_x, &is_solid) in row.iter().enumerate() {
                    if is_solid && grid[ny as usize + cell_y][nx as usize + cell_x] {
                        if dy == -1 {
                            break 'falling;
                        } else {
                            continue 'slide_and_drop;
                        }
                    }
                }
            }
            x = nx;
            y = ny;
        }
    }
    for (cell_y, row) in shape.iter().enumerate() {
        for (cell_x, &is_solid) in row.iter().enumerate() {
            if is_solid {
                grid[y as usize + cell_y][x as usize + cell_x] = true;
            }
        }
    }
    *height = max(*height, y + shape_height);
}

fn get_height(jets: &[i64], shapes: &[Vec<Vec<bool>>], num_rocks: i64) -> i64 {
    let mut grid = vec![vec![false; WIDTH]; MAX_HEIGHT];
    let mut time = 0;
    let mut height = 0;
    let mut index = 0;
    let mut past_states = HashMap::new();
    let mut additional_height = 0;
    while index < num_rocks {
        let shape = &shapes[index as usize % shapes.len()];
        drop_rock(jets, shape, &mut grid, &mut time, &mut height);
        let top_grid = if height >= NUM_TOP_ROWS {
            grid[(height - NUM_TOP_ROWS) as usize..height as usize]
                .iter()
                .cloned()
                .collect_vec()
        } else {
            vec![]
        };
        let state = (
            time % jets.len() as i64,
            index as usize % shapes.len(),
            top_grid.clone(),
        );
        if let Some(&(old_index, old_height, _)) = past_states.get(&state) {
            if additional_height == 0 {
                let index_diff = index - old_index;
                let height_diff = height - old_height;
                let repeats = (num_rocks - old_index) / index_diff - 1;
                index += repeats * index_diff;
                additional_height = repeats * height_diff;
            }
        } else {
            past_states.insert(state, (index, height, top_grid.clone()));
        }
        index += 1;
    }
    height + additional_height
}

pub fn run(input: &str) {
    let jets = input
        .bytes()
        .map(|b| if b == b'<' { -1 } else { 1 })
        .collect_vec();
    let shapes = [
        "####",          // -
        ".#.\n###\n.#.", // +
        "..#\n..#\n###", // ┛
        "#\n#\n#\n#",    // |
        "##\n##",        // ■
    ]
    .iter()
    .map(|shape| {
        shape
            .lines()
            .map(|row| row.bytes().map(|b| b == b'#').collect_vec())
            .rev()
            .collect_vec()
    })
    .collect_vec();

    let height = get_height(&jets, &shapes, 2022);
    println!("{}", height);

    let height = get_height(&jets, &shapes, 1_000_000_000_000);
    println!("{}", height);
}
