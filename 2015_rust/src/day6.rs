use std::borrow::Borrow;
use std::io::prelude::*;
use std::io;

pub fn main() {
    let mut bf = Bitfield::new(1000);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line_str = line.borrow();

        let cmd = Command::parse(line_str);
        // println!("command: {:?}", cmd);
        run_cmd(&cmd, &mut bf);
    }

    print_ones(&bf, bf.size);
}

fn print_ones(bf : &Bitfield, size : u64) {
    let mut ones = 0;
    for i in 0 .. size * size {
        if bf.check_bit(i) {
            ones += 1;
        }
    }
    println!("{}", ones);
}

fn run_cmd(cmd : &Command, bf : &mut Bitfield) {
    let mut bits : Vec<u64> = Vec::new();
    for x_bit in cmd.x_start .. cmd.x_end + 1 {
        for y_bit in cmd.y_start .. cmd.y_end + 1 {
            bits.push(bf.to_bit_idx(x_bit, y_bit));
        }
    }

    match cmd.cty {
        CommandType::TurnOn => {
            for bit_idx in bits {
                bf.set_bit(bit_idx);
            }
        },
        CommandType::TurnOff => {
            for bit_idx in bits {
                bf.unset_bit(bit_idx);
            }
        },
        CommandType::Toggle => {
            for bit_idx in bits {
                if bf.check_bit(bit_idx) {
                    bf.unset_bit(bit_idx);
                } else {
                    bf.set_bit(bit_idx);
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Commands

#[derive(PartialEq, Debug)]
enum CommandType {
    TurnOn, TurnOff, Toggle
}

#[derive(Debug)]
struct Command {
    cty : CommandType,
    x_start : u64,
    x_end : u64,
    y_start : u64,
    y_end : u64
}

impl Command {
    fn parse(s : &str) -> Command {
        let words : Vec<&str> = s.split_whitespace().collect();

        let cty =
            if words[0] == "turn" {
                if words[1] == "on" {
                    CommandType::TurnOn
                } else {
                    CommandType::TurnOff
                }
            } else {
                CommandType::Toggle
            };

        let start_str =
            if cty == CommandType::TurnOn || cty == CommandType::TurnOff {
                words[2]
            } else {
                words[1]
            };

        let start_comma = start_str.find(',').unwrap();
        let (start_f, start_s) = start_str.split_at(start_comma);
        let start_s = &start_s[1..];

        let start_idx_x = start_f.parse().unwrap();
        let start_idx_y = start_s.parse().unwrap();

        let end_str =
            if cty == CommandType::TurnOn || cty == CommandType::TurnOff {
                words[4]
            } else {
                words[3]
            };

        let end_comma = end_str.find(',').unwrap();
        let (end_f, end_s) = end_str.split_at(end_comma);
        let end_s = &end_s[1..];

        let end_idx_x = end_f.parse().unwrap();
        let end_idx_y = end_s.parse().unwrap();

        Command { cty: cty,
                  x_start: start_idx_x,
                  y_start: start_idx_y,
                  x_end: end_idx_x,
                  y_end: end_idx_y }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Bitfield stuff

/// A 2D bitfield.
struct Bitfield {
    bitfield: Vec<u8>,
    size: u64,
}

impl Bitfield {
    fn new(size : u64) -> Bitfield {
        let num_bits = size * size;
        let mut bytes = num_bits / 8;
        let leftovers = num_bits % 8;

        if leftovers != 0 {
            bytes += 1;
        }

        let field = vec![0; bytes as usize];
        Bitfield { bitfield: field, size: size }
    }

    fn check_bit(&self, bit : u64) -> bool {
        let byte_idx = bit / 8;
        let bit_idx = bit % 8;
        let byte : u8 = self.bitfield[byte_idx as usize];
        (byte >> bit_idx) & 0b00000001 == 1
    }

    fn set_bit(&mut self, bit : u64) {
        let byte_idx = bit / 8;
        let bit_idx = bit % 8;
        let byte = self.bitfield[byte_idx as usize];
        let new_byte = byte | (0b00000001 << bit_idx);
        self.bitfield[byte_idx as usize] = new_byte;
    }

    fn unset_bit(&mut self, bit : u64) {
        let byte_idx = bit / 8;
        let bit_idx = bit % 8;
        let byte = self.bitfield[byte_idx as usize];
        self.bitfield[byte_idx as usize] = byte & !(0b00000001 << bit_idx);
    }

    fn show(&self) -> String {
        let mut ret = String::new();

        for bit in 0 .. (self.bitfield.len() * 8) {
            if self.check_bit(bit as u64) {
                ret.push('1');
            } else {
                ret.push('0');
            }
        }

        ret
    }

    fn to_bit_idx(&self, x : u64, y : u64) -> u64 {
        y * self.size + x
    }
}

