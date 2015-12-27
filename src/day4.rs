// I'm not sure if there's a way other than brute-force search for this.
// Doing brute-force.

use std::borrow::Borrow;
use std::char;
use std::io::Write;
use std::io;
use std::str::FromStr;
use std::str;
use std::string::String;

extern crate md5;

pub fn main() {
    /*
    let test : String = "pqrstuv1048970".to_string();
    let digest : [u8; 16] = md5::compute(test.into_bytes().borrow());
    println!("{} {}", digest_str(&digest), five_zeroes(&digest));

    let test : String = "abcdef609043".to_string();
    let digest : [u8; 16] = md5::compute(test.into_bytes().borrow());
    println!("{} {}", digest_str(&digest), five_zeroes(&digest));
    */

    print!("Secret key: ");
    io::stdout().flush().unwrap();

    let mut secret_key = String::new();
    io::stdin().read_line(&mut secret_key).unwrap();
    let secret_key_ : &str = secret_key.trim();

    // println!("Secret key: {}-", secret_key_);

    let mut i : i64 = 0;
    loop {
        let i_str = i.to_string();

        let mut ans : String = secret_key_.to_string();
        ans.push_str(i_str.borrow());

        // println!("Trying {}", ans);

        let digest : [u8; 16] = md5::compute(ans.into_bytes().borrow());

        if five_zeroes(&digest) {
            let s = digest_str(&digest);
            println!("Found it: {} {}", i, s);
            break;
        }

        i += 1;
    }
}

fn five_zeroes(digest : &[u8; 16]) -> bool {
    let char_1_2 = digest[0];
    let char_3_4 = digest[1];
    let char_5   = digest[2] >> 4;

    char_1_2 == 0 && char_3_4 == 0 && char_5 == 0
}

fn digest_str(digest : &[u8; 16]) -> String {
    let mut vec : Vec<char> = Vec::new();

    for byte in digest.into_iter() {
        let (char1, char2) = byte_char(*byte);
        vec.push(char1);
        vec.push(char2);
    }

    vec.iter().cloned().collect::<String>()
}

fn byte_char(byte : u8) -> (char, char) {
    let first  = (byte & 0b11110000) >> 4;
    let second = byte & 0b00001111;
    (char::from_digit(first as u32, 16).unwrap(), char::from_digit(second as u32, 16).unwrap())
}
