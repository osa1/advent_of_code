use std::io::prelude::*;
use std::io;

pub fn main() {
    let mut grid : Grid = Grid::from_stdin();

    for _ in 0 .. 100 {
        let mut next : Grid = Grid::all_off();
        next_state(&grid, &mut next);
        grid = next.clone();
    }

    println!("{}", grid.lights_on());
}

// We could use a bitfield like we did in Day 6.
struct Grid {
    grid: [bool; 100 * 100],
}

#[inline]
fn to_grid_idx(x : usize, y : usize) -> usize {
    100 * y + x
}

// because rustc is stupid and can't figure how to clone an array
impl Clone for Grid {
    fn clone(&self) -> Grid {
        Grid { grid: self.grid }
    }
}

impl Grid {
    fn from_stdin() -> Grid {
        let mut grid : [bool; 100 * 100] = [false; 100 * 100];

        let stdin = io::stdin();

        for (line_idx, line) in stdin.lock().lines().enumerate() {
            for (col_idx, ch) in line.unwrap().chars().enumerate() {
                if ch == '#' {
                    grid[ to_grid_idx(col_idx, line_idx) ] = true;
                } else if ch == '.' {
                    // nothing to do, the grid is already initialized with all
                    // bits off
                } else {
                    panic!("Don't recognize char: {}", ch)
                }
            }
        }

        Grid { grid: grid }
    }

    fn all_off() -> Grid {
        Grid { grid: [false; 100 * 100] }
    }

    #[inline]
    fn light_at(&self, x : usize, y : usize) -> bool {
        self.grid[ to_grid_idx(x, y) ]
    }

    fn on(&mut self, x : usize, y : usize) {
        self.grid[ to_grid_idx(x, y) ] = true;
    }

    fn off(&mut self, x : usize, y : usize) {
        self.grid[ to_grid_idx(x, y) ] = false;
    }

    fn lights_on(&self) -> u64 {
        let mut ret = 0;
        for bit in self.grid.iter() {
            if *bit {
                ret += 1;
            }
        }
        ret
    }

    fn lights_on_around(&self, x : usize, y : usize) -> u64 {
        let mut ret = 0;

        let x = x as i64;
        let y = y as i64;

        let idxs = [ (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                     (x - 1, y),                 (x + 1, y),
                     (x - 1, y + 1), (x, y + 1), (x + 1, y + 1) ];

        for xy in idxs.iter() {
            let (x, y) = *xy;
            if x >= 0 && x < 100 && y >= 0 && y < 100 {
                if self.grid[ to_grid_idx(x as usize, y as usize) ] {
                    ret += 1;
                }
            }
        }

        ret
    }
}

fn next_state(grid : &Grid, next : &mut Grid) {
    for y in 0 .. 100 {
        for x in 0 .. 100 {
            let ons = grid.lights_on_around(x, y);
            if grid.light_at(x, y) {
                if ons == 2 || ons == 3 {
                    next.on(x, y);
                } else {
                    next.off(x, y);
                }
            } else {
                if ons == 3 {
                    next.on(x, y);
                } else {
                    next.off(x, y);
                }
            }
        }
    }
}
