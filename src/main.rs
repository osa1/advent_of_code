mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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
    }
}
