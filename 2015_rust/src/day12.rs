use std::io::prelude::*;
use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let chars : Vec<char> = input.chars().collect();

    let mut i = 0;
    let mut ret = 0;

    while i < chars.len() {
        if chars[i] == '-' {
            match try_read_int(&chars, i + 1) {
                None => { i += 2; },
                Some(int_str) => {
                    ret -= int_str.parse().unwrap();
                    i += int_str.len() + 1;
                }
            }
        } else if chars[i].is_numeric() {
            match try_read_int(&chars, i) {
                None => {
                    // This case is impossible. Why? Because we know there's at
                    // least one numeric char.
                    panic!("How did that happen?");
                },
                Some(int_str) => {
                    ret += int_str.parse().unwrap();
                    i += int_str.len();
                }
            }
        } else {
            i += 1;
        }
    }

    println!("{}", ret);
}

fn try_read_int(chars : &Vec<char>, mut idx : usize) -> Option<String> {
    let mut ret = String::new();

    while idx < chars.len() {
        if chars[idx].is_numeric() {
            ret.push(chars[idx]);
            idx += 1;
        } else {
            break;
        }
    }

    if ret.len() == 0 {
        None
    } else {
        Some(ret)
    }
}
