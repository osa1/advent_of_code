const INPUT: i32 = 325489;

// (x increment, y increment, step increment)
// right, up, left down
static DIR: [(i32, i32, i32); 4] = [(1, 0, 1), (0, -1, 0), (-1, 0, 1), (0, 1, 0)];

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut step = 0;
    let mut input = INPUT;
    'outer:
    loop {
        for &(x_inc, y_inc, step_inc) in DIR.iter() {
            step += step_inc;
            // println!("step_inc: {}, step: {}", step_inc, step);
            for _ in 0 .. step {
                x += x_inc;
                y += y_inc;
                // println!("x: {}, y: {}", x, y);
                input -= 1;
                if input == 1 {
                    break 'outer;
                }
            }
        }
    }

    println!("x: {}, y: {}", x, y);
}

fn part2() {
    use std::collections::HashMap;

    let mut values: HashMap<(i32, i32), i32> = HashMap::new();
    values.insert((0, 0), 1);

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut step = 0;
    let mut input = INPUT;
    'outer:
    loop {
        for &(x_inc, y_inc, step_inc) in DIR.iter() {
            step += step_inc;
            // println!("step_inc: {}, step: {}", step_inc, step);
            for _ in 0 .. step {
                x += x_inc;
                y += y_inc;

                let mut val = 0;
                val += values.get(&(x+1, y)).unwrap_or(&0);
                val += values.get(&(x+1, y+1)).unwrap_or(&0);
                val += values.get(&(x, y+1)).unwrap_or(&0);
                val += values.get(&(x-1, y+1)).unwrap_or(&0);
                val += values.get(&(x-1, y)).unwrap_or(&0);
                val += values.get(&(x-1, y-1)).unwrap_or(&0);
                val += values.get(&(x, y-1)).unwrap_or(&0);
                val += values.get(&(x+1, y-1)).unwrap_or(&0);
                values.insert((x, y), val);

                if val >= INPUT {
                    println!("val: {}", val);
                    return;
                }

                input -= 1;
                if input == 1 {
                    break 'outer;
                }
            }
        }
    }
}
