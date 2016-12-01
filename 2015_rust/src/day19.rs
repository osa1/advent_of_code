use std::io::prelude::*;
use std::io;
use std::collections::HashSet;

pub fn main() {
    let mut input : String = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut molecule : Option<&str> = None;
    let mut rewrites : Vec<(&str, &str)> = Vec::new();

    for line in input.lines() {
        let words : Vec<&str> = line.split_whitespace().collect();

        if words.len() == 1 {
            assert!(molecule == None);
            molecule = Some(words[0]);
        } else if words.len() == 3 {
            rewrites.push((words[0], words[2]));
        } else {
            println!("Skipping line: {:?}", line);
        }
    }

    // println!("rewrites {:?}", rewrites);
    // println!("{}", count_rewrites(&rewrites, molecule.unwrap()));
    println!("{:?}", replacements(&rewrites, molecule.unwrap()).len());
}

fn replacements(rewrites : &Vec<(&str, &str)>, molecule : &str) -> HashSet<String> {
    let mut ret = HashSet::new();

    for rewrite in rewrites {
        let (pat, new) = *rewrite;
        for (start, part) in molecule.match_indices(pat) {
            let mut rw = String::new();
            rw.push_str(unsafe { molecule.slice_unchecked(0, start) });
            rw.push_str(new);
            rw.push_str(unsafe { molecule.slice_unchecked(start + part.len(), molecule.len()) });
            ret.insert(rw);
        }
    }

    ret
}
