use std::collections::HashSet;

pub fn run() {
    let mut banks: Vec<i32> = {
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace().map(|s| s.parse().unwrap()).collect()
    };

    println!("{:?}", distribute(&mut banks, None));

    // Part 2: loop again, until seeing the current state again
    let current_state = banks.clone();
    println!("{:?}", distribute(&mut banks, Some(current_state)));
}

fn distribute(banks: &mut Vec<i32>, target: Option<Vec<i32>>) -> i32 {
    let mut steps = 0;
    let mut seen_states: HashSet<Vec<i32>> = HashSet::new();

    loop {
        steps += 1;

        // two iterations but whatever
        let mut max: i32 = banks.iter().max().unwrap().clone();
        let max_idx: usize = banks.iter().position(|i| *i == max).unwrap();

        // distribute bank
        let mut idx = (max_idx + 1) % banks.len();
        banks[max_idx] = 0;
        while max > 0 {
            max -= 1;
            banks[idx] += 1;
            idx = (idx + 1) % banks.len();
        }

        if let Some(ref target) = target {
            if banks == target {
                break;
            }
        } else if seen_states.contains(banks) {
            break;
        } else {
            seen_states.insert(banks.clone());
        }
    }

    steps
}
