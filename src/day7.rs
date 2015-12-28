use std::borrow::Borrow;
use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut gates: Vec<Gate> = Vec::new();

    for line in input.lines() {
        gates.push(parse_gate(line));
    }

    let mut results = HashMap::new();
    println!("output of gate a: {}", gen_output_s(&gates, "a", &mut results));
}

fn gen_output<'a>(gates : &'a Vec<Gate<'a>>,
                  gate : &'a GateInput,
                  results: &mut HashMap<&'a str, i64>) -> i64 {
    match *gate {
        GateInput::Imm(i) => i,
        GateInput::AnotherGate(ref s) => gen_output_s(gates, s.borrow(), results)
    }
}

fn gen_output_s<'a>(gates : &'a Vec<Gate>,
                    gate : &'a str,
                    results : &mut HashMap<&'a str, i64>) -> i64 {

    match results.get(gate) {
        Some(i) => return *i,
        _ => ()
    }

    // println!("results: {:?}", results);

    let input = gates.iter().find(|g| g.output == gate);
    match input {
        None => panic!("Can't find gate {}", gate),
        Some(i) => {
            let ret = match i.gate_fn {
                GateFn::And(ref input1, ref input2) =>
                    gen_output(gates, input1, results) & gen_output(gates, input2, results),
                GateFn::Or(ref input1, ref input2) =>
                    gen_output(gates, input1, results) | gen_output(gates, input2, results),
                GateFn::LShift(ref input1, ref input2) =>
                    gen_output(gates, input1, results) << gen_output(gates, input2, results),
                GateFn::RShift(ref input1, ref input2) =>
                    gen_output(gates, input1, results) >> gen_output(gates, input2, results),
                GateFn::Id(ref input) =>
                    gen_output(gates, input, results),
                GateFn::Not(ref input) =>
                    ! gen_output(gates, input, results)
            };
            // println!("inserting result for {}: {}", gate, ret);
            results.insert(gate, ret);
            ret
        }
    }
}

#[derive(Debug, Clone)]
enum GateInput<'a> {
    Imm(i64),
    AnotherGate(&'a str)
}

type GateOutput<'a> = &'a str;

#[derive(Debug, Clone)]
enum GateFn<'a> {
    And(GateInput<'a>, GateInput<'a>),
    Or(GateInput<'a>, GateInput<'a>),
    LShift(GateInput<'a>, GateInput<'a>),
    RShift(GateInput<'a>, GateInput<'a>),
    Id(GateInput<'a>),
    Not(GateInput<'a>),
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    gate_fn: GateFn<'a>,
    output: GateOutput<'a>,
}

fn parse_gate(s : &str) -> Gate {
    let words : Vec<&str> = s.split_whitespace().collect();

    if words[0] == "NOT" {
        Gate { gate_fn: GateFn::Not(parse_input(words[1])),
               output: words[3] }
    } else if words[1] == "AND" {
        Gate { gate_fn: GateFn::And(parse_input(words[0]), parse_input(words[2])),
               output: words[4] }
    } else if words[1] == "OR" {
        Gate { gate_fn: GateFn::Or(parse_input(words[0]), parse_input(words[2])),
               output: words[4] }
    } else if words[1] == "LSHIFT" {
        Gate { gate_fn: GateFn::LShift(parse_input(words[0]), parse_input(words[2])),
               output: words[4] }
    } else if words[1] == "RSHIFT" {
        Gate { gate_fn: GateFn::RShift(parse_input(words[0]), parse_input(words[2])),
               output: words[4] }
    } else {
        Gate { gate_fn: GateFn::Id(parse_input(words[0])),
               output: words[2] }
    }
}

fn parse_input(s : &str) -> GateInput {
    if s.chars().nth(0).unwrap().is_numeric() {
        GateInput::Imm(s.parse().unwrap())
    } else {
        GateInput::AnotherGate(s)
    }
}
