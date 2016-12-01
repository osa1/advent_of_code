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
mod day14;
mod day18;
mod day19;
mod day21;
mod day23;
mod day24;
mod day25;

use std::env;
use std::iter::FromIterator;

fn unimplemented() {
    println!("Not implemented yet.");
}

static DAYS : [ fn(); 25 ] =
    [ day1::main, day2::main, day3::main, day4::main, day5::main, day6::main, day7::main,
      day8::main, day9::main, day10::main, day11::main, day12::main, day13::main, day14::main,
      unimplemented/*day15::main*/, unimplemented/*day16::main*/, unimplemented/*day17::main*/,
      day18::main, day19::main, unimplemented/*day20::main*/, day21::main,
      unimplemented/*day22::main*/, day23::main, day24::main, day25::main ];

fn main() {
    let args = Vec::from_iter(env::args_os());
    let arg : i64 = args[1].clone().into_string().unwrap().parse().unwrap();

    DAYS[arg as usize - 1]();
}
