use std::cmp;
use std::io::prelude::*;
use std::io;
use std::iter::FromIterator;

pub fn main() {
    let stdin = io::stdin();

    let mut ret : i64 = 0;

    for line in stdin.lock().lines() {
        let line_ = line.unwrap();
        let vec = Vec::from_iter(line_.split('x'));

        let l : i64 = vec[0].parse().unwrap();
        let w : i64 = vec[1].parse().unwrap();
        let h : i64 = vec[2].parse().unwrap();

        let area1 = l * w;
        let area2 = l * h;
        let area3 = w * h;

        ret += 2 * area1 + 2 * area2 + 2 * area3 + cmp::min(cmp::min(area1, area2), area3);
    }

    println!("{}", ret);
}
