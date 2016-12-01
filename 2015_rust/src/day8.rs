use std::io::prelude::*;
use std::io;

pub fn main() {
    let stdin = io::stdin();

    let mut code_chars = 0;
    let mut string_chars = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();

        println!("reading line: {}", line);

        let chars : Vec<char> = line.chars().collect();

        // Skip initial "
        let mut char_idx = 1;
        // -1 for skipping last "
        while char_idx < line.len() - 1 {
            let ch = chars[char_idx];
            if ch == '\\' {
                let next_ch = chars[char_idx + 1];
                if next_ch == '\\' || next_ch == '"' {
                    string_chars += 1;
                    char_idx += 2;
                } else if next_ch == 'x' {
                    string_chars += 1;
                    char_idx += 4;
                } else {
                    panic!("Unrecognized escape char: {} (at {})", next_ch, char_idx + 1);
                }
            } else {
                string_chars += 1;
                char_idx += 1;
            }
        }

        // We didn't count the last ", so adding 1 here
        code_chars += char_idx + 1;
    }

    println!("code_chars: {}, string_chars: {}, code_chars - string_chars: {}",
             code_chars, string_chars, code_chars - string_chars);
}
