use std::collections::HashMap;
use std::io::BufRead;

pub fn run() {
    let mut regs: HashMap<String, i64> = HashMap::new();
    let mut max_reg_val: Option<i64> = None;

    let stdin = ::std::io::stdin();
    for line in stdin.lock().lines().map(Result::unwrap) {
        let mut words = line.split_whitespace();

        let reg1 = words.next().unwrap();
        let inc_dec = words.next().unwrap();
        let amt: i64 = words.next().unwrap().parse().unwrap();
        let if_ = words.next().unwrap();
        debug_assert!(if_ == "if");
        let reg2 = words.next().unwrap();
        let op = words.next().unwrap();
        let num: i64 = words.next().unwrap().parse().unwrap();

        let reg2_val = regs.get(reg2).cloned().unwrap_or(0);

        let cond = match op {
            "<" =>
                reg2_val < num,
            ">" =>
                reg2_val > num,
            "<=" =>
                reg2_val <= num,
            ">=" =>
                reg2_val >= num,
            "==" =>
                reg2_val == num,
            "!=" =>
                reg2_val != num,
            _ =>
                panic!("Unknown operator: {}", op),
        };

        if cond {
            match inc_dec {
                "inc" => {
                    *regs.entry(reg1.to_owned()).or_insert(0) += amt;
                },
                "dec" => {
                    *regs.entry(reg1.to_owned()).or_insert(0) -= amt;
                },
                _ => {
                    panic!("Unknown operation: {}", inc_dec);
                },
            }
        }

        if max_reg_val.is_none() || regs.get(reg1).cloned() > max_reg_val {
            max_reg_val = Some(regs.get(reg1).cloned().unwrap());
        }
    }

    let mut max_keyval = None;
    for (k, v) in regs.iter() {
        if max_keyval == None {
            max_keyval = Some((k, v))
        } else if v > max_keyval.unwrap().1 {
            max_keyval = Some((k, v))
        }
    }

    println!("{:?}", max_keyval);
    println!("{:?}", max_reg_val);
}
