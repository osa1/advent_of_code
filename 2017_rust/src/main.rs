mod day1;

fn main() {
    let mut args = ::std::env::args();
    args.next().unwrap();
    let day_str: String = args.next().unwrap();
    let day = day_str.parse::<u8>().unwrap();

    match day {
        1 =>
            day1::run(),
        _ =>
            panic!("Day {} not implemented yet", day),
    }
}
