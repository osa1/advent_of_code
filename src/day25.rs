pub fn main() {
    let input_row = 3010;
    let input_col = 3019;
    let mut idx = idx_diagonal(input_row, input_col);

    let mut code = FIRST_CODE;
    while idx != 0 {
        code = next_code(code);
        idx -= 1;
    }

    println!("{}", code);
}

const FIRST_CODE : u64 = 20151125;
const MULT : u64 = 252533;
const DIV : u64 = 33554393;

fn next_code(prev_code : u64) -> u64 {
    (prev_code * MULT) % DIV
}

// Ops.. We actually need the inverse of this..
// fn diagonal_idx(mut i : u64) -> (u64, u64) {
//     let mut current_row = 1;
//     let mut current_col = 1;
//     while i > 1 {
//         if current_row == 1 {
//             current_row = current_col + 1;
//             current_col = 1;
//         } else {
//             current_row -= 1;
//             current_col += 1;
//         }
//         i -= 1;
//     }
//
//     (current_row, current_col)
// }

fn idx_diagonal(mut row : u64, mut col : u64) -> u64 {
    let mut ret = 0;

    while !(row == 1 && col == 1) {
        if col == 1 {
            col = row - 1;
            row = 1;
        } else {
            col -= 1;
            row += 1;
        }
        ret += 1;
    }

    ret
}
