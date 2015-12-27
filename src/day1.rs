use std::io;

pub fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("{}", run(input)),
        Err(error) => println!("error: {}", error),
    }
}

fn run(input: String) -> i64 {
    let mut ret = 0;

    for c in input.chars() {
        if c == '(' {
            ret += 1;
        } else if c == ')' {
            ret -= 1;
        }
    }

    return ret;
}
