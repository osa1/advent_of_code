// This problem is essentially a combination generation problem. One fun thing
// about this problem is that it's actually easier than it sounds, beucase we
// know that the input has a solution - e.g. we don't need to check other groups
// once we found a group with sum = sum(input) / 3.
//
// So, we start with group with one element, if not found, continue with 2, 3,
// 4 ... elements. If multiple groups are found with same number of elements, we
// look at the product and choose the one with smallest product.

use std::borrow::Borrow;
use std::cmp;
use std::io::BufRead;
use std::io;

fn combs(start : usize, end : usize, comb_size : usize) -> Vec< Vec<usize> > {
    debug_assert!(comb_size <= end - start);
    if comb_size == 0 {
        vec![vec![]]
    } else {
        let mut ret : Vec< Vec<usize> > = Vec::new();
        for i in start .. end - comb_size + 1 {
            let mut rest : Vec< Vec<usize> > = combs(i + 1, end, comb_size - 1);
            for comb in rest.iter_mut() {
                comb.push(i);
            }
            ret.extend_from_slice(rest.borrow());
        }
        ret
    }
}

pub fn main() {
    // println!("{:?}", combs(0, 5, 3));
    let stdin = io::stdin();
    let ws : Vec<u64> = stdin.lock().lines().map(|l| l.unwrap().parse().unwrap()).collect();

    // I don't understand how can Iterator::sum() not be stable. What's there to
    // be stable?
    let group_total : u64 = ws.iter().fold(0, |a, b| a + b) / 3;

    // Try to find groups with 1 element, if not found, try 2 elements etc.
    // If multiple groups are found, pick one with smallest product.
    // -2 because there should be two elements left for the other two groups.
    for gs in 1 .. ws.len() - 2 {
        println!("Searching groups of size {}", gs);
        let comb_idxss = combs(0, ws.len(), gs);

        let mut min_prod : Option<u64> = None;

        for comb_idxs in comb_idxss {
            let sum : u64 = comb_idxs.iter().map(|idx| ws[*idx]).fold(0, |a, b| a + b);
            let prod : u64 = comb_idxs.iter().map(|idx| ws[*idx]).fold(1, |a, b| a * b);
            if sum == group_total {
                match min_prod {
                    None => { min_prod = Some(prod); },
                    Some(r) => { min_prod = Some(cmp::min(r, prod)); }
                }
            }
        }

        if let Some(ret) = min_prod {
            println!("{}", ret);
            break;
        }
    }
}
