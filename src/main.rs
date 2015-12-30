extern crate ascii;
extern crate permutohedron; // for generating permutations

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day21;
mod day23;
mod day25;

use std::env;
use std::iter::FromIterator;

fn main() {
    let args = Vec::from_iter(env::args_os());
    let arg : i64 = args[1].clone().into_string().unwrap().parse().unwrap();

    if arg == 1 {
        day1::main();
    } else if arg == 2 {
        day2::main();
    } else if arg == 3 {
        day3::main();
    } else if arg == 4 {
        day4::main();
    } else if arg == 5 {
        day5::main();
    } else if arg == 6 {
        day6::main();
    } else if arg == 7 {
        day7::main();
    } else if arg == 8 {
        day8::main();
    } else if arg == 9 {
        day9::main();
    } else if arg == 10 {
        day10::main();
    } else if arg == 11 {
        day11::main();
    } else if arg == 12 {
        day12::main();
    } else if arg == 13 {
        day13::main();
    } else if arg == 21 {
        day21::main();
    } else if arg == 23 {
        day23::main();
    } else if arg == 25 {
        day25::main();
    }
}
