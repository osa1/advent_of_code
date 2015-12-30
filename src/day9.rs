// This problem must be even harder than the "Hamiltonian path" problem, which
// is only concerned with finding a path - it doesn't care about the
// length/weight/etc. or paths.
//
// Given that Hamiltonian path is NP-complete, I'd expect this to be NP-complete
// too.
//
// In our case the input is very small, so a brute-force algorithm should do the
// trick.

use permutohedron::Heap;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;
use std::io;

type Map<'a> = HashMap<&'a str, HashMap<&'a str, u64>>;

pub fn main() {
    let mut input : String = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut map : Map = HashMap::new();
    let mut nodes : HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let words : Vec<&str> = line.split_whitespace().collect();
        let from = words[0];
        let to = words[2];
        let cost : u64 = words[4].parse().unwrap();

        map.entry(from).or_insert_with(HashMap::new).insert(to, cost);
        map.entry(to).or_insert_with(HashMap::new).insert(from, cost);

        nodes.insert(from);
        nodes.insert(to);
    }

    let mut nodes_vec : Vec<&str> = nodes.into_iter().collect();
    let perm_heap = Heap::new(&mut nodes_vec);

    let mut min_cost : Option<u64> = None;
    for perm in perm_heap {
        let cost = cost(&perm, &map);
        match min_cost {
            None => { min_cost = Some(cost); },
            Some(current) => { min_cost = Some(cmp::min(current, cost)); }
        }
    }

    println!("{:?}", min_cost);
}

fn cost(v : &Vec<&str>, map : &Map) -> u64 {
    let mut ret = 0;

    for i in 0 .. v.len() - 1 {
        let node1 = v[i];
        let node2 = v[i + 1];
        ret += *map.get(node1).unwrap().get(node2).unwrap();
    }

    ret
}
