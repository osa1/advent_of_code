use std::cmp;
use std::io::prelude::*;
use std::io;
use std::iter::Iterator;

pub fn main() {
    let stdin = io::stdin();

    let mut rs : Vec<Reindeer> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let words : Vec<&str> = line.split_whitespace().collect();

        let name = words[0];
        let speed = words[3].parse().unwrap();
        let fly_duration = words[6].parse().unwrap();
        let must_rest = words[13].parse().unwrap();

        rs.push(Reindeer {
            name: String::from(name),
            speed: speed,
            fly_duration: fly_duration,
            must_rest: must_rest,
        });
    }

    println!("{:?}", rs);

    let bestest = rs.iter().map(|rd| distance_traveled(rd, 2503)).max().unwrap();
    println!("bestest: {}", bestest);
}

#[derive(Debug)]
struct Reindeer {
    name : String,
    speed : u64,
    fly_duration : u64,
    must_rest : u64
}

fn distance_traveled(rd : &Reindeer, time : u64) -> u64 {
    let cycles = time / (rd.fly_duration + rd.must_rest);
    let rest   = time % (rd.fly_duration + rd.must_rest);
    let rest   = cmp::min(rd.fly_duration, rest);

    (cycles * rd.fly_duration + rest) * rd.speed
}
