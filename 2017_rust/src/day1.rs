pub fn run() {
    let mut input = String::new();
    ::std::io::stdin().read_line(&mut input).unwrap();

    // drop last byte (newline)
    let input_bytes = (&input[0 .. input.len() - 1]).as_bytes();
    let num_digits = input_bytes.len();

    let mut sum: u64 = 0;
    for i in 0 .. num_digits {
        let digit1 = input_bytes[i] - b'0';
        let digit2 = input_bytes[(i + num_digits / 2) % num_digits] - b'0';
        if digit1 == digit2 {
            sum += digit1 as u64
        }
    }

    println!("{}", sum);
}
