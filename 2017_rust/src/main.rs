#![feature(entry_and_modify)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let mut args = ::std::env::args();
    args.next().unwrap();
    let day_str: String = args.next().unwrap();
    let day = day_str.parse::<u8>().unwrap();

    match day {
        1 =>
            day1::run(),
        2 =>
            day2::run(),
        3 =>
            day3::run(),
        4 =>
            day4::run(),
        5 =>
            day5::run(),
        6 =>
            day6::run(),
        _ =>
            panic!("Day {} not implemented yet", day),
    }
}
