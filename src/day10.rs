fn total_signal_strength(x_values: &[i32]) -> i32 {
    (20..=220).step_by(40).map(|cycle| cycle as i32 * x_values[cycle - 1]).sum()
}

fn render_image(x_values: &[i32]) {
    for y in 0..6 {
        for x in 0..40 {
            let cycle = y * 40 + x;
            let x_val = x_values[cycle as usize];
            if ((x - 1)..=(x + 1)).contains(&x_val) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub fn run(input: &str) {
    let mut x = 1;
    let mut x_values = vec![x];
    input.lines().for_each(|operation| {
        x_values.push(x);
        match operation {
            "noop" => {}
            _ => {
                let value = operation.split(' ').last().unwrap().parse::<i32>().unwrap();
                x += value;
                x_values.push(x);
            }
        }
    });

    let signal_strength = total_signal_strength(&x_values);
    println!("{}", signal_strength);

    render_image(&x_values);
}
