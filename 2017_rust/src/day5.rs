use std::io::BufRead;

pub fn run() {
    let mut jumps: Vec<i32> = vec![];

    let stdin = ::std::io::stdin();
    for line in stdin.lock().lines().map(Result::unwrap) {
        jumps.push(line.parse().unwrap());
    }

    let jumps_len = jumps.len() as i32;
    let mut steps = 0;
    let mut idx = 0;
    while idx >= 0 && idx < jumps_len {
        let offset = jumps[idx as usize];
        let target = idx + offset;
        jumps[idx as usize] += if offset >= 3 { -1 } else { 1 };
        idx = target;
        steps += 1;
    }

    println!("steps: {}", steps);
}
