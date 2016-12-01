// This is the first program I used ASCII strings.
//
// This program led to this patch:
// https://github.com/tomprogrammer/rust-ascii/issues/12

use ascii::Ascii;
use ascii::AsciiStr;
use ascii::AsciiString;
use std::borrow::Borrow;
use std::io::Read;
use std::io;

pub fn main() {
    let input : Vec<u8> =
        io::stdin().bytes().collect::<Result<Vec<u8>, io::Error>>().unwrap();

    let str : &AsciiStr = AsciiStr::from_bytes(&input).unwrap().trim();

    if str.len() != 8 {
        panic!("Input length should be 8 (found {}, {})", str.len(), str);
    }

    // Another feature request: AsciiString::from_str(&AsciiStr)
    let mut pass = AsciiString::new();
    pass.push_str(str);

    while ! (check_increasing(pass.borrow())
             && check_forbidden(pass.borrow())
             && check_doubles(pass.borrow())) {
        pass = next_pass(pass.borrow());
        // println!("next pass: {}", pass);
    }

    println!("{}", pass);
}

// Passwords must include one increasing straight of at least three letters,
// like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd
// doesn't count.
fn check_increasing(s : &AsciiStr) -> bool {
    for i in 0 .. s.len() - 2 {
        let char1 = s[i];
        let char2 = s[i + 1];
        let char3 = s[i + 2];

        if next_char(char1) == char2 && next_char(char2) == char3 {
            return true;
        }
    }

    false
}

// Passwords may not contain the letters i, o, or l, as these letters can be
// mistaken for other characters and are therefore confusing.
fn check_forbidden(s : &AsciiStr) -> bool {
    let i = Ascii::from('i').unwrap();
    let o = Ascii::from('o').unwrap();
    let l = Ascii::from('l').unwrap();

    // What?? Can't iterate over a AsciiStr?? Maybe it's time for another
    // feature request..
    for idx in 0 .. s.len() {
        let c = s[idx];
        if c == i || c == o || c == l {
            return false;
        }
    }

    true
}

// Passwords must contain at least two different, non-overlapping pairs of
// letters, like aa, bb, or zz.
fn check_doubles(s : &AsciiStr) -> bool {
    let mut first_double : Option<Ascii> = None;

    for i in 0 .. s.len() - 1 {
        if s[i] == s[i + 1] {
            match first_double {
                None => { first_double = Some(s[i]); },
                Some(c) => {
                    if c != s[i] {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn next_pass(s : &AsciiStr) -> AsciiString {
    let mut new : AsciiString = AsciiString::new();
    new.push_str(s);
    incr_char(&mut new, s.len() - 1);
    new
}

fn incr_char(s : &mut AsciiString, i : usize) {
    if s[i].as_char() == 'z' {
        s[i] = Ascii::from('a').unwrap();
        if i != 0 {
            incr_char(s, i - 1);
        }
    } else {
        s[i] = next_char(s[i])
    }
}

fn next_char(c : Ascii) -> Ascii {
    if c.as_char() == 'z' {
        Ascii::from('a').unwrap()
    } else {
        Ascii::from_byte(c.as_byte() + 1).unwrap()
    }
}
