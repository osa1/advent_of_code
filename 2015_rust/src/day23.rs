use std::borrow::Borrow;
use std::io::prelude::*;
use std::io;

pub fn main() {
    let stdin = io::stdin();

    let mut instrs = Vec::new();
    for line in stdin.lock().lines() {
        instrs.push(parse_instr(line.unwrap().borrow()));
    }

    // println!("{:?}", instrs);
    println!("{:?}", run_instrs(&instrs));
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
enum Reg { RegA, RegB }

#[derive(Debug)]
enum Arg { Reg(Reg), Imm(i64) }

#[derive(Debug)]
enum Instr {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(i64),
    Jie(Reg, i64),
    Jio(Reg, i64),
}

////////////////////////////////////////////////////////////////////////////////
// The interpreter

fn run_instrs(instrs : &Vec<Instr>) -> (i64, i64) {
    let instrs_slice : &[Instr] = instrs.borrow();
    let mut ip = 0;

    let mut reg_a = 0;
    let mut reg_b = 0;

    while ip < instrs.len() {
        match instrs_slice[ip] {
            Instr::Hlf(Reg::RegA) => { reg_a /= 2; ip += 1; },
            Instr::Hlf(Reg::RegB) => { reg_b /= 2; ip += 1; },

            Instr::Tpl(Reg::RegA) => { reg_a *= 3; ip += 1; },
            Instr::Tpl(Reg::RegB) => { reg_b *= 3; ip += 1; },

            Instr::Inc(Reg::RegA) => { reg_a += 1; ip += 1; },
            Instr::Inc(Reg::RegB) => { reg_b += 1; ip += 1; },

            Instr::Jmp(offset) => { ip += offset as usize; },

            Instr::Jie(ref reg, offset) => {
                let val = match *reg {
                    Reg::RegA => reg_a,
                    Reg::RegB => reg_b
                };

                let ip_inc =
                    if val % 2 == 0 {
                        offset as usize
                    } else {
                        1
                    };

                ip += ip_inc;
            },

            Instr::Jio(ref reg, offset) => {
                let val = match *reg {
                    Reg::RegA => reg_a,
                    Reg::RegB => reg_b
                };

                if val == 1 {
                    ip += offset as usize;
                } else {
                    ip += 1;
                }
            }
        }
    }

    (reg_a, reg_b)
}

////////////////////////////////////////////////////////////////////////////////
// Parsing

fn parse_instr(chars : &str) -> Instr {
    let (instr_str, rest) = chars.split_at(3);
    let args : Vec<Arg> = rest.split(',').map(|s| parse_arg(s.trim())).collect();

    if instr_str == "hlf" {
        Instr::Hlf(get_reg_arg(&args[0]))
    } else if instr_str == "tpl" {
        Instr::Tpl(get_reg_arg(&args[0]))
    } else if instr_str == "inc" {
        Instr::Inc(get_reg_arg(&args[0]))
    } else if instr_str == "jmp" {
        Instr::Jmp(get_imm_arg(&args[0]))
    } else if instr_str == "jie" {
        Instr::Jie(get_reg_arg(&args[0]), get_imm_arg(&args[1]))
    } else if instr_str == "jio" {
        Instr::Jio(get_reg_arg(&args[0]), get_imm_arg(&args[1]))
    } else {
        panic!("Unknown instruction: {}", instr_str)
    }
}

fn get_reg_arg(arg : &Arg) -> Reg {
    match *arg {
        Arg::Imm(_) => panic!("Found immediate argument: {:?}", arg),
        Arg::Reg(ref r) => r.clone()
    }
}

fn get_imm_arg(arg : &Arg) -> i64 {
    match *arg {
        Arg::Imm(i) => i,
        Arg::Reg(_) => panic!("Found reg argument: {:?}", arg)
    }
}

fn parse_arg(chars : &str) -> Arg {
    let ch = chars.chars().nth(0).unwrap();
    if ch == 'a' {
        Arg::Reg(Reg::RegA)
    } else if ch == 'b' {
        Arg::Reg(Reg::RegB)
    } else {
        Arg::Imm(chars.parse().unwrap())
    }
}
