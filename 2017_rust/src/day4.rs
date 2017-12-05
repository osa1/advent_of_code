use std::collections::HashMap;
use std::io::BufRead;

pub fn run() {
    let mut valid: i32 = 0;

    let stdin = ::std::io::stdin();
    'line_loop:
    for line in stdin.lock().lines() {
        let mut word_bags: Vec<HashMap<char, i32>> = vec![];
        for word in line.unwrap().split_whitespace() {
            let mut char_map = HashMap::new();
            for char in word.chars() {
                char_map.entry(char).and_modify(|i| *i += 1).or_insert(1);
            }
            for bag in &word_bags {
                if *bag == char_map {
                    continue 'line_loop;
                }
            }
            word_bags.push(char_map);
        }
        valid += 1;
    }

    println!("{}", valid);
}
