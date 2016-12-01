use std::io;
use std::collections::HashSet;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut visited_locations : HashSet<(i64, i64)> = HashSet::new();

    let mut current_location_x = 0;
    let mut current_location_y = 0;

    visited_locations.insert((current_location_x, current_location_y));

    for c in input.chars() {
        if c == '>' {
            current_location_x += 1;
        } else if c == '<' {
            current_location_x -= 1;
        } else if c == '^' {
            current_location_y += 1;
        } else if c == 'v' {
            current_location_y -= 1;
        }
        visited_locations.insert((current_location_x, current_location_y));
    }

    println!("{}", visited_locations.len());
}
