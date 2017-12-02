use std::io::BufRead;

pub fn run() {
    let stdin = ::std::io::stdin();
    let mut ret = 0;
    for line in stdin.lock().lines().map(Result::unwrap) {
        let mut nums = vec![];
        for word in line.split_whitespace() {
            nums.push(str::parse::<i64>(word).unwrap());
        }
        'l:
        for i in 0 .. nums.len() - 1 {
            for j in i + 1 .. nums.len() {
                let ni = nums[i];
                let nj = nums[j];
                let min = ::std::cmp::min(ni, nj);
                let max = ::std::cmp::max(ni, nj);
                if max % min == 0 {
                    ret += max / min;
                    break 'l;
                }
            }
        }
    }
    println!("{}", ret);
}
