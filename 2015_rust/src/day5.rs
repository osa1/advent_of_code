use std::borrow::Borrow;
use std::io::prelude::*;
use std::io;

pub fn main() {
    let stdin = io::stdin();

    let mut ret = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line_str = line.borrow();
        if three_vowels(line_str) &&
                does_not_contain(line_str, "ab") &&
                does_not_contain(line_str, "cd") &&
                does_not_contain(line_str, "pq") &&
                does_not_contain(line_str, "xy") &&
                repeats(line_str) {
            ret += 1;
        }
    }

    println!("{}", ret);
}

fn three_vowels(s : &str) -> bool {
    let mut vowels = 0;

    for c in s.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowels += 1;
            if vowels == 3 {
                return true;
            }
        }
    }

    false
}

fn does_not_contain(s : &str, forbidden : &str) -> bool {
    !s.contains(forbidden)
}

fn repeats(s : &str) -> bool {
    for i in 1 .. s.len() {
        if s.chars().nth(i).unwrap() == s.chars().nth(i - 1).unwrap() {
            return true;
        }
    }
    false
}
