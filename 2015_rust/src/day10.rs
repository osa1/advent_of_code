use std::borrow::Borrow;
use std::io::prelude::*;
use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = String::from(input.trim());

    for _ in 0 .. 40 {
        let chars : Vec<char> = input.chars().collect();
        let mut idx = 0;

        let mut next_input = String::new();

        while idx < chars.len() {
            let count = 1 + count(&chars, idx + 1, chars[idx]);
            next_input.push_str(count.to_string().borrow());
            next_input.push(chars[idx]);
            idx += count as usize;
        }

        // println!("next input: {}", next_input);
        input = next_input;
    }

    println!("{}", input.len());
}

fn count(str : &Vec<char>, mut idx : usize, c : char) -> u64 {
    let mut ret = 0;
    while idx < str.len() && str[idx] == c {
        ret += 1;
        idx += 1;
    }
    ret
}
