// This is essentially the same with Day 9. Only differences are:
//
// - We need to calculate costs by adding up costs in both directions.
// - We need to take the cost from last node to first one into account.
//
// Can't use ascii library here, because it sucks. (no lines() etc.)
// Ideally, the library would provide the API of str for AsciiStr and of String
// for AsciiString.

use permutohedron::Heap;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;
use std::io;

type Map<'a> = HashMap<&'a str, HashMap<&'a str, i64>>;

pub fn main() {
    let mut input : String = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut map : Map = HashMap::new();
    let mut nodes : HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let words : Vec<&str> = line.split_whitespace().collect();
        let from = words[0];
        let to = words[10];

        // Drop '.' . I'm sure it's one byte here.
        let to : &str = &to[0 .. to.len() - 1];

        let mut cost : i64 = words[3].parse().unwrap();

        if words[2] == "lose" {
            cost = -cost;
        }

        map.entry(from).or_insert_with(HashMap::new).insert(to, cost);

        nodes.insert(from);
        nodes.insert(to);
    }

    // println!("{:?}", nodes);

    let mut nodes_vec : Vec<&str> = nodes.into_iter().collect();
    let perm_heap = Heap::new(&mut nodes_vec);

    let mut max_cost : Option<i64> = None;
    for perm in perm_heap {
        let cost = cost(&perm, &map);
        match max_cost {
            None => { max_cost = Some(cost); },
            Some(current) => { max_cost = Some(cmp::max(current, cost)); }
        }
    }

    println!("{:?}", max_cost);
}

fn cost(v : &Vec<&str>, map : &Map) -> i64 {
    let mut ret = 0;

    for i in 0 .. v.len() - 1 {
        let node1 = v[i];
        let node2 = v[i + 1];
        ret += *map.get(node1).unwrap().get(node2).unwrap();
        ret += *map.get(node2).unwrap().get(node1).unwrap();
    }

    ret += *map.get(v[0]).unwrap().get(v[ v.len() - 1 ]).unwrap();
    ret += *map.get(v[ v.len() - 1 ]).unwrap().get(v[0]).unwrap();

    ret
}
