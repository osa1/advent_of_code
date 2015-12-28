mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day10;

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
    } else if arg == 10 {
        day10::main();
    }
}
