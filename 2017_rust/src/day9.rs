use std::io::Read;

pub fn run() {
    let mut input = String::new();
    ::std::io::stdin().read_to_string(&mut input).unwrap();
    println!("{:?}", score(&input));
}

// (score, garbage)
fn score(input: &str) -> (usize, usize) {
    let mut garbage = false;
    let mut nesting: usize = 0;
    let mut score: usize = 0;
    let mut garbage_amt = 0;

    let mut chars = input.chars();
    while let Some(char) = chars.next() {

        if garbage && char != '>' && char != '!' {
            garbage_amt += 1;
        }

        match char {
            '{' => {
                if !garbage {
                    nesting += 1;
                    score += nesting;
                }
            },
            '}' => {
                if !garbage {
                    assert!(nesting > 0);
                    nesting -= 1;
                }
            },
            '<' => {
                garbage = true;
            },
            '>' => {
                garbage = false;
            },
            '!' => {
                chars.next();
            },
            _ => {}
        }
    }

    assert_eq!(nesting, 0);
    (score, garbage_amt)
}

#[test]
fn test() {
    assert_eq!(score("{}").0, 1);
    assert_eq!(score("{{{}}}").0, 6);
    assert_eq!(score("{{},{}}").0, 5);
    assert_eq!(score("{{{},{},{{}}}}").0, 16);
    assert_eq!(score("{<a>,<a>,<a>,<a>}").0, 1);
    assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
    assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
    assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);

    assert_eq!(score("<>").1, 0);
    assert_eq!(score("<random characters>").1, 17);
    assert_eq!(score("<<<<>").1, 3);
    assert_eq!(score("<{!>}>").1, 2);
    assert_eq!(score("<!!>").1, 0);
    assert_eq!(score("<!!!>>").1, 0);
    assert_eq!(score("<{o\"i!a,<{i<a>").1, 10);
}
